use cosmwasm_std::{entry_point, Addr, Binary, Deps, StdResult};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw2::{get_contract_version, set_contract_version};
use localmoney_protocol::denom_utils::denom_to_string;

use crate::state::{ADMIN, CONFIG};
use localmoney_protocol::errors::ContractError;
use localmoney_protocol::errors::ContractError::Unauthorized;
use localmoney_protocol::guards::assert_migration_parameters;
use localmoney_protocol::hub::{
    Admin, ExecuteMsg, HubConfig, InstantiateMsg, MigrateMsg, QueryMsg,
};
use localmoney_protocol::offer::ExecuteMsg::RegisterHub as OfferRegisterHub;
use localmoney_protocol::price::ExecuteMsg::RegisterHub as PriceRegisterHub;
use localmoney_protocol::profile::ExecuteMsg::RegisterHub as ProfileRegisterHub;
use localmoney_protocol::trade::ExecuteMsg::RegisterHub as TradeRegisterHub;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();

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
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(config) => update_config(deps, info, config),
        ExecuteMsg::UpdateAdmin { admin_addr } => update_admin(deps, info, admin_addr),
    }
}

fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    config: HubConfig,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage).unwrap();
    if !info.sender.eq(&admin.addr) {
        return Err(Unauthorized {
            owner: admin.addr.clone(),
            caller: info.sender.clone(),
        });
    }
    CONFIG.save(deps.storage, &config).unwrap();

    let offer_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.offer_addr.to_string(),
        msg: to_binary(&OfferRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let price_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.price_addr.to_string(),
        msg: to_binary(&PriceRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let profile_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.profile_addr.to_string(),
        msg: to_binary(&ProfileRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let trade_register_hub = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.trade_addr.to_string(),
        msg: to_binary(&TradeRegisterHub {}).unwrap(),
        funds: info.funds.clone(),
    }));

    let res = Response::new()
        .add_attribute("action", "update_config")
        .add_submessage(offer_register_hub)
        .add_submessage(price_register_hub)
        .add_submessage(profile_register_hub)
        .add_submessage(trade_register_hub)
        .add_attribute("local_market_addr", config.local_market_addr)
        .add_attribute("offer_addr", config.offer_addr)
        .add_attribute("price_addr", config.price_addr)
        .add_attribute("profile_addr", config.profile_addr)
        .add_attribute("trade_addr", config.trade_addr);
    Ok(res)
}

fn update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Addr,
) -> Result<Response, ContractError> {
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
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let previous_contract_version = get_contract_version(deps.storage).unwrap();

    assert_migration_parameters(
        previous_contract_version.clone(),
        CONTRACT_NAME.to_string(),
        CONTRACT_VERSION,
    )
    .unwrap();

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    // If the structure of the data in storage changes, we must treat it here

    Ok(Response::default()
        .add_attribute("previous_version", previous_contract_version.version)
        .add_attribute("new_version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
