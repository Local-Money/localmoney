use cosmwasm_std::{entry_point, Addr, Binary, Deps, StdResult};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw20::Denom;

use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::Unauthorized;
use localterra_protocol::hub::{Admin, ExecuteMsg, HubConfig, InstantiateMsg, QueryMsg};
use localterra_protocol::kujira::msg::KujiraMsg;
use localterra_protocol::kujira::query::KujiraQuery;
use localterra_protocol::offer::ExecuteMsg::RegisterHub as OfferRegisterHub;
use localterra_protocol::profile::ExecuteMsg::RegisterHub as ProfileRegisterHub;
use localterra_protocol::trade::ExecuteMsg::RegisterHub as TradeRegisterHub;
use localterra_protocol::trade::MigrateMsg;
use localterra_protocol::trading_incentives::ExecuteMsg::RegisterHub as TradeIncentivesRegisterHub;

use crate::state::{ADMIN, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    let admin = Admin {
        addr: msg.admin_addr.clone(),
    };
    ADMIN.save(deps.storage, &admin).unwrap();

    let res = Response::new()
        .add_attribute("action", "instantiate_hub")
        .add_attribute("admin_addr", msg.admin_addr.to_string());
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(config) => update_config(deps, info, config),
        ExecuteMsg::UpdateAdmin { admin_addr } => update_admin(deps, info, admin_addr),
    }
}

fn update_config(
    deps: DepsMut<KujiraQuery>,
    info: MessageInfo,
    config: HubConfig,
) -> Result<Response<KujiraMsg>, ContractError> {
    let admin = ADMIN.load(deps.storage).unwrap();
    if !info.sender.eq(&admin.addr) {
        return Err(Unauthorized {
            owner: admin.addr.clone(),
            caller: info.sender.clone(),
        });
    }
    CONFIG.save(deps.storage, &config).unwrap();
    let local_denom = match config.local_denom {
        Denom::Native(s) => s,
        Denom::Cw20(addr) => addr.to_string(),
    };

    let offer_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.offer_addr.to_string(),
        msg: to_binary(&OfferRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let trade_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.trade_addr.to_string(),
        msg: to_binary(&TradeRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let trading_incentives_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.trading_incentives_addr.to_string(),
        msg: to_binary(&TradeIncentivesRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let profile_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.profile_addr.to_string(),
        msg: to_binary(&ProfileRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let res = Response::new()
        .add_attribute("action", "update_config")
        .add_submessage(offer_register_hub)
        .add_submessage(trade_register_hub)
        .add_submessage(trading_incentives_register_hub)
        .add_submessage(profile_register_hub)
        .add_attribute("local_denom", local_denom)
        .add_attribute("local_market_addr", config.local_market_addr)
        .add_attribute("offer_addr", config.offer_addr)
        .add_attribute("trade_addr", config.trade_addr)
        .add_attribute("trading_incentives_addr", config.trading_incentives_addr);

    Ok(res)
}

fn update_admin(
    deps: DepsMut<KujiraQuery>,
    info: MessageInfo,
    new_admin: Addr,
) -> Result<Response<KujiraMsg>, ContractError> {
    let mut admin = ADMIN.load(deps.storage).unwrap();
    if !info.sender.eq(&admin.addr) {
        return Err(Unauthorized {
            owner: admin.addr.clone(),
            caller: info.sender.clone(),
        });
    }

    let old_admin = admin.addr.clone();
    admin.addr = new_admin.clone();
    ADMIN.save(deps.storage, &admin).unwrap();

    let res = Response::new()
        .add_attribute("action", "update_admin")
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut<KujiraQuery>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    Ok(Response::default())
}
