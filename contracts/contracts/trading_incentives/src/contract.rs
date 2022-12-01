use std::cmp;
use std::ops::{Add, Mul};

use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, QueryRequest,
    StdResult, Storage, SubMsg, WasmQuery,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20::Denom;

use localmoney_protocol::errors::ContractError;
use localmoney_protocol::errors::ContractError::{
    DistributionClaimInvalidPeriod, DistributionNotStarted, HubAlreadyRegistered,
    InvalidTradeState, Unauthorized,
};
use localmoney_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localmoney_protocol::offer::TradeInfo;
use localmoney_protocol::trade::{QueryMsg as TradeQueryMsg, TradeState};
use localmoney_protocol::trading_incentives::{
    Distribution, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, TraderRewards,
};

use crate::state::{DISTRIBUTION, TOTAL_VOLUME, TRADER_VOLUME};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let period_duration = 604800u64; //1 week in seconds
    let distribution_periods = 51u8;
    let total_duration = period_duration * distribution_periods as u64;
    let tokens_per_period = Uint128::zero();

    DISTRIBUTION
        .save(
            deps.storage,
            &Distribution {
                start_time: 0u64,
                end_time: 0,
                period_duration,
                periods: distribution_periods,
                current_period: 0,
                tokens_per_period,
            },
        )
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "instantiate_trading_incentives")
        .add_attribute("period_duration", period_duration.to_string())
        .add_attribute("distribution_periods", distribution_periods.to_string())
        .add_attribute("total_duration", total_duration.to_string());

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterTrade { trade } => register_trade(deps, env, info, trade),
        ExecuteMsg::ClaimRewards { period } => claim_rewards(deps, env, info, period),
        ExecuteMsg::StartDistribution {} => start_distribution(deps, env, info),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Distribution {} => to_binary(&get_distribution_info(env, deps.storage)?),
        QueryMsg::Rewards { trader, period } => {
            to_binary(&get_rewards(deps.storage, trader, period)?)
        }
    }
}

fn get_distribution_info(env: Env, storage: &dyn Storage) -> StdResult<Distribution> {
    let mut distribution = DISTRIBUTION.load(storage).unwrap();

    //Calculate the current period.
    let block_time = env.block.time.clone();
    let period = (block_time.seconds() - distribution.start_time) / distribution.period_duration;
    let period = cmp::min(period as u8, distribution.periods);

    //Calculate the end time.
    let end_time =
        distribution.start_time + (distribution.period_duration * distribution.periods as u64);

    distribution.end_time = end_time;
    distribution.current_period = period;

    Ok(distribution)
}

fn get_rewards(storage: &dyn Storage, trader: String, period: u8) -> StdResult<TraderRewards> {
    let distribution = DISTRIBUTION.load(storage).unwrap();

    let total_volume = TOTAL_VOLUME.load(storage, &[period]).unwrap();
    let trader_volume = TRADER_VOLUME
        .load(storage, (trader.as_bytes(), &[period]))
        .unwrap();

    let trader_share = trader_volume / total_volume;
    let trader_rewards = TraderRewards {
        amount: distribution.tokens_per_period.mul(trader_share),
    };
    Ok(trader_rewards)
}

fn register_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let hub_cfg = get_hub_config(deps.as_ref());

    //Only callable by the Trade Contract.
    if hub_cfg.trade_addr.ne(&info.sender) {
        return Err(Unauthorized {
            owner: hub_cfg.trade_addr.clone(),
            caller: info.sender.clone(),
        });
    }

    let trade_info: TradeInfo = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: hub_cfg.trade_addr.to_string(),
            msg: to_binary(&TradeQueryMsg::Trade {
                id: trade_id.clone(),
            })
            .unwrap(),
        }))
        .unwrap();

    let trade = trade_info.trade;
    let offer_response = trade_info.offer;
    let maker = offer_response.offer.owner.to_string();

    if trade.state != TradeState::EscrowReleased {
        return Err(InvalidTradeState {
            current: trade.state.clone(),
            expected: TradeState::EscrowReleased,
        });
    }

    let amount = trade.amount.clone();
    let distribution_info = get_distribution_info(env.clone(), deps.storage).unwrap();
    let period = distribution_info.current_period;

    let total_volume_store = TOTAL_VOLUME.key(&[period]);
    let mut total_volume = total_volume_store
        .load(deps.storage)
        .unwrap_or(Uint128::zero());
    total_volume = total_volume.add(amount);
    total_volume_store
        .save(deps.storage, &total_volume)
        .unwrap();

    let trader_volume_store = TRADER_VOLUME.key((&maker.as_bytes(), &[period]));
    let trader_volume = trader_volume_store
        .load(deps.storage)
        .unwrap_or(Uint128::zero());
    trader_volume_store
        .save(deps.storage, &trader_volume.add(amount.clone()))
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "register_trade")
        .add_attribute("trade_id", trade_id)
        .add_attribute("maker", maker)
        .add_attribute("amount", amount)
        .add_attribute("period", period.to_string());

    Ok(res)
}

fn claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    period: u8,
) -> Result<Response, ContractError> {
    let distribution = get_distribution_info(env, deps.storage).unwrap();
    let rewards_denom = get_rewards_denom(deps.as_ref());

    if distribution.start_time.eq(&0u64) {
        return Err(DistributionNotStarted {});
    }

    if period >= distribution.current_period {
        return Err(DistributionClaimInvalidPeriod {});
    }

    let amount =
        get_rewards(deps.storage, info.sender.to_string(), period).unwrap_or(TraderRewards {
            amount: Uint128::zero(),
        });
    let res = Response::new()
        .add_submessage(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: rewards_denom.clone(),
                amount: amount.amount.clone(),
            }],
        })))
        .add_attribute("action", "claim_rewards")
        .add_attribute("maker", info.sender)
        .add_attribute("amount", amount.amount.to_string())
        .add_attribute("denom", rewards_denom)
        .add_attribute("period", period.to_string());

    Ok(res)
}

fn start_distribution(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut distribution = DISTRIBUTION.load(deps.storage).unwrap();
    let rewards_denom = get_rewards_denom(deps.as_ref());

    let rewards = info
        .funds
        .iter()
        .find(|c| c.denom.eq(&rewards_denom))
        .unwrap();

    let tokens_per_period = Decimal::from_ratio(rewards.amount, distribution.periods);
    distribution.start_time = env.block.time.seconds();
    distribution.tokens_per_period = Uint128::new(1u128).mul(tokens_per_period);
    DISTRIBUTION.save(deps.storage, &distribution).unwrap();

    let res = Response::new()
        .add_attribute("action", "start_distribution")
        .add_attribute("denom", rewards_denom)
        .add_attribute(
            "tokens_per_period",
            distribution.tokens_per_period.to_string(),
        )
        .add_attribute("distribution_periods", distribution.periods.to_string())
        .add_attribute("amount", rewards.amount);

    Ok(res)
}

fn get_rewards_denom(deps: Deps) -> String {
    let hub_cfg = get_hub_config(deps.clone());
    match hub_cfg.local_denom {
        Denom::Native(name) => name,
        Denom::Cw20(addr) => addr.to_string(),
    }
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
