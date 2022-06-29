use cosmwasm_std::OverflowOperation::Add;
use cosmwasm_std::{
    entry_point, Addr, Binary, ContractResult, Deps, Reply, ReplyOn, StdError, StdResult,
    SubMsgResponse,
};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw20::Denom;

use crate::errors::FactoryError;
use crate::errors::FactoryError::Unauthorized;
use crate::state::{ADMIN, CONFIG};
use localterra_protocol::factory::{Admin, Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use localterra_protocol::factory_util::get_contract_address_from_reply;
use localterra_protocol::offer::InstantiateMsg as OfferInstantiate;
use localterra_protocol::trading_incentives::InstantiateMsg as TradingIncentivesInstantiateMsg;

pub const OFFER_REPLY_ID: u64 = 2;
pub const TRADING_INCENTIVES_REPLY_ID: u64 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, FactoryError> {
    let admin = Admin {
        addr: msg.admin_addr.clone(),
    };
    ADMIN.save(deps.storage, &admin).unwrap();

    let res = Response::new().add_attribute("admin_addr", msg.admin_addr.to_string());
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, FactoryError> {
    match msg {
        ExecuteMsg::UpdateConfig(config) => update_config(deps, info, config),
        ExecuteMsg::UpdateAdmin { admin_addr } => update_admin(deps, info, admin_addr),
    }
}

fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    config: Config,
) -> Result<Response, FactoryError> {
    let admin = ADMIN.load(deps.storage).unwrap();
    if !info.sender.eq(&admin.addr) {
        return Err(Unauthorized {});
    }
    CONFIG.save(deps.storage, &config).unwrap();
    let local_denom = match config.local_denom {
        Denom::Native(s) => s,
        Denom::Cw20(addr) => addr.to_string(),
    };
    let res = Response::default()
        .add_attribute("local_denom", local_denom)
        .add_attribute("local_market_addr", config.local_market_addr)
        .add_attribute("offer_addr", config.offer_addr)
        .add_attribute("trade_addr", config.trade_addr)
        .add_attribute("trading_incentives_addr", config.trading_incentives_addr);

    Ok(res)
}

fn update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Addr,
) -> Result<Response, FactoryError> {
    let mut admin = ADMIN.load(deps.storage).unwrap();
    if !info.sender.eq(&admin.addr) {
        return Err(Unauthorized {});
    }

    let old_admin = admin.addr.clone();
    admin.addr = new_admin.clone();
    ADMIN.save(deps.storage, &admin).unwrap();

    let res = Response::default()
        .add_attribute("old_admin", old_admin)
        .add_attribute("new_admin", new_admin);
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage).unwrap()),
        QueryMsg::Admin {} => to_binary(&ADMIN.load(deps.storage).unwrap()),
    }
}
