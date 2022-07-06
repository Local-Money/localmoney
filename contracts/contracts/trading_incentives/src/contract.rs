use crate::errors::TradingIncentivesError;
use crate::errors::TradingIncentivesError::HubAlreadyRegistered;
use crate::math::DECIMAL_FRACTIONAL;
use crate::state::{DISTRIBUTION, TOTAL_VOLUME, TRADER_VOLUME};
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, QueryRequest,
    StdError, StdResult, Storage, SubMsg, WasmQuery,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20::Denom;
use localterra_protocol::hub_util::{get_hub_config, register_hub_internal, HUB_ADDR};
use localterra_protocol::trade::{QueryMsg as TradeQueryMsg, Trade, TradeState};
use localterra_protocol::trading_incentives::{Distribution, ExecuteMsg, InstantiateMsg, QueryMsg};
use std::cmp;
use std::ops::{Add, Mul};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, TradingIncentivesError> {
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
) -> Result<Response, TradingIncentivesError> {
    match msg {
        ExecuteMsg::RegisterTrade { trade, maker } => register_trade(deps, env, info, trade, maker),
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

fn get_rewards(storage: &dyn Storage, trader: String, period: u8) -> StdResult<Uint128> {
    let distribution = DISTRIBUTION.load(storage).unwrap();

    let total_volume = TOTAL_VOLUME.load(storage, &[period]).unwrap();
    let trader_volume = TRADER_VOLUME
        .load(storage, (trader.as_bytes(), &[period]))
        .unwrap();

    let trader_share = Decimal::from_ratio(trader_volume, total_volume) / DECIMAL_FRACTIONAL;
    Ok(distribution.tokens_per_period.mul(trader_share))
}

fn register_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    maker: String,
) -> Result<Response, TradingIncentivesError> {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    let hub_cfg = get_hub_config(&deps.querier, hub_addr.addr.to_string());

    //Only callable by the Trade Contract.
    if hub_cfg.trade_addr.ne(&info.sender) {
        return Err(TradingIncentivesError::Unauthorized {});
    }

    let maker = deps
        .api
        .addr_validate(&maker.as_str())
        .unwrap()
        .into_string();

    let trade: Trade = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: hub_cfg.trade_addr.to_string(),
            msg: to_binary(&TradeQueryMsg::Trade {
                id: trade_id.clone(),
            })
            .unwrap(),
        }))
        .unwrap();

    if trade.state != TradeState::EscrowReleased {
        return Err(TradingIncentivesError::Unauthorized {});
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
    _env: Env,
    info: MessageInfo,
    period: u8,
) -> Result<Response, TradingIncentivesError> {
    let distribution = DISTRIBUTION.load(deps.storage).unwrap();

    if distribution.start_time.eq(&0u64) {
        return Err(TradingIncentivesError::Std(StdError::generic_err(
            "Distribution hasn't started yet.",
        )));
    }

    if period >= distribution.current_period {
        return Err(TradingIncentivesError::Std(StdError::generic_err(
            "Only past periods can be claimed.",
        )));
    }

    let amount =
        get_rewards(deps.storage, info.sender.to_string(), period).unwrap_or(Uint128::zero());
    let res = Response::new()
        .add_submessage(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: "".to_string(),
                amount: amount.clone(),
            }],
        })))
        .add_attribute("action", "claim")
        .add_attribute("maker", info.sender)
        .add_attribute("amount", amount)
        .add_attribute("period", period.to_string());

    Ok(res)
}

fn start_distribution(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, TradingIncentivesError> {
    let mut distribution = DISTRIBUTION.load(deps.storage).unwrap();
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    let hub_cfg = get_hub_config(&deps.querier, hub_addr.addr.to_string());

    let rewards_denom = match hub_cfg.local_denom {
        Denom::Native(name) => name,
        Denom::Cw20(addr) => addr.to_string(),
    };

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

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, TradingIncentivesError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}
