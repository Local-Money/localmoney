use crate::errors::TradingIncentivesError;
use crate::errors::TradingIncentivesError::HubAlreadyRegistered;
use crate::math::DECIMAL_FRACTIONAL;
use crate::state::{CONFIG, TOTAL_VOLUME, TRADER_VOLUME};
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, StdError, StdResult,
    Storage, SubMsg,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20::Denom;
use localterra_protocol::factory_util::{
    get_factory_config, register_hub_internal, HubConfig, HUB_CONFIG,
};
use localterra_protocol::trading_incentives::{
    Config, Distribution, ExecuteMsg, InstantiateMsg, QueryMsg,
};
use std::cmp;
use std::ops::Mul;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, TradingIncentivesError> {
    let period_duration = 604800u64; //1 week in seconds
    let distribution_periods = 51u8;
    let total_duration = period_duration * distribution_periods as u64;
    let distribution_start = 0u64;

    let tokens_per_period = Uint128::zero();

    CONFIG
        .save(
            deps.storage,
            &Config {
                factory_addr: info.sender.clone(),
                distribution_start,
                distribution_period_duration: total_duration,
                distribution_periods,
                tokens_per_period,
            },
        )
        .unwrap();
    let res = Response::new()
        .add_attribute("action", "instantiate_gov")
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
        ExecuteMsg::RegisterTrade { trade, maker } => register_trade(trade, maker, deps, env),
        ExecuteMsg::Claim { period } => claim(deps, env, info, period),
        ExecuteMsg::StartDistribution => start_distribution(deps, env, info),
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
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn get_distribution_info(env: Env, storage: &dyn Storage) -> StdResult<Distribution> {
    let cfg = CONFIG.load(storage).unwrap();
    let block_time = env.block.time.clone();
    let period = (block_time.seconds() - cfg.distribution_start) / cfg.distribution_period_duration;
    let period = cmp::min(period as u8, cfg.distribution_periods);

    let end_time = cfg.distribution_start
        + (cfg.distribution_period_duration * cfg.distribution_periods as u64);
    Ok(Distribution {
        distribution_start_time: cfg.distribution_start,
        distribution_end_time: end_time,
        period_duration: cfg.distribution_period_duration,
        current_period: period,
        tokens_per_period: cfg.tokens_per_period,
    })
}

fn get_rewards(storage: &dyn Storage, trader: String, period: u8) -> StdResult<Uint128> {
    let cfg = CONFIG.load(storage).unwrap();

    let total_volume = TOTAL_VOLUME.load(storage, &[period]).unwrap();
    let trader_volume = TRADER_VOLUME
        .load(storage, (trader.as_bytes(), &[period]))
        .unwrap();

    let trader_share = Decimal::from_ratio(trader_volume, total_volume) / DECIMAL_FRACTIONAL;
    Ok(cfg.tokens_per_period.mul(trader_share))
}

fn register_trade(
    _trade: String,
    _maker: String,
    _deps: DepsMut,
    _env: Env,
) -> Result<Response, TradingIncentivesError> {
    //TODO: Refactor
    Ok(Response::default())
    /*
    let cfg = CONFIG.load(deps.storage).unwrap();

    let trade = deps
        .api
        .addr_validate(&trade.as_str())
        .unwrap()
        .into_string();

    let maker = deps
        .api
        .addr_validate(&maker.as_str())
        .unwrap()
        .into_string();

    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.into_string());
    let trade_info: TradeInfo = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: factory_cfg.offers_addr.into_string(),
            msg: to_binary(&OfferQueryMsg::TradesStates { trades: vec![] }).unwrap(),
        }))
        .unwrap();

    if trade_info.trade.state != TradeTradeState::Released {
        return Err(TradingIncentivesError::Unauthorized {});
    }

    let amount = trade_info.trade.amount.amount.clone();
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
        .add_attribute("trade", trade)
        .add_attribute("maker", maker)
        .add_attribute("amount", amount);

    Ok(res)
    */
}

fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    period: u8,
) -> Result<Response, TradingIncentivesError> {
    let cfg = CONFIG.load(deps.storage).unwrap();
    let distribution_info = get_distribution_info(env.clone(), deps.storage).unwrap();
    let current_period = distribution_info.current_period.clone();

    if cfg.distribution_start.eq(&0u64) {
        return Err(TradingIncentivesError::Std(StdError::generic_err(
            "Distribution hasn't started yet.",
        )));
    }

    if period >= current_period {
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
    let mut cfg = CONFIG.load(deps.storage).unwrap();
    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());
    let rewards_denom = match factory_cfg.local_denom {
        Denom::Native(name) => name,
        Denom::Cw20(addr) => addr.to_string(),
    };
    let rewards = info
        .funds
        .iter()
        .find(|c| c.denom.eq(&rewards_denom))
        .unwrap();
    cfg.distribution_start = env.block.time.seconds();
    cfg.tokens_per_period = Uint128::new(1u128)
        .mul(Decimal::from_ratio(rewards.amount, cfg.distribution_periods) / DECIMAL_FRACTIONAL);
    CONFIG.save(deps.storage, &cfg).unwrap();
    Ok(Response::default())
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, TradingIncentivesError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_config(deps: Deps) -> StdResult<HubConfig> {
    let cfg = HUB_CONFIG.load(deps.storage).unwrap();
    Ok(cfg)
}
