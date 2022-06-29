use crate::errors::GuardError;
use crate::errors::GuardError::HubAlreadyRegistered;
use crate::factory::{Config, QueryMsg};
use cosmwasm_std::{
    to_binary, Addr, ContractResult, Deps, DepsMut, MessageInfo, QuerierWrapper, QueryRequest,
    Response, SubMsgResponse, WasmQuery,
};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_factory_config(querier: &QuerierWrapper, factory_addr: String) -> Config {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: factory_addr,
            msg: to_binary(&QueryMsg::Config {}).unwrap(),
        }))
        .unwrap()
}

pub fn get_contract_address_from_reply(deps: Deps, result: ContractResult<SubMsgResponse>) -> Addr {
    result
        .unwrap()
        .events
        .into_iter()
        .find(|e| e.ty == "instantiate")
        .and_then(|ev| {
            ev.attributes
                .into_iter()
                .find(|attr| attr.key == "_contract_address")
        })
        .map(|attr| deps.api.addr_validate(attr.value.as_str()).unwrap())
        .unwrap()
}

pub fn register_hub_internal(
    hub_addr: Addr,
    mut hub_config_storage: Singleton<HubConfig>,
) -> Result<Response, GuardError> {
    let cfg = hub_config_storage.may_load().unwrap();
    if cfg.is_some() {
        return Err(HubAlreadyRegistered {});
    }
    hub_config_storage
        .save(&HubConfig {
            hub_addr: hub_addr.clone(),
        })
        .unwrap();
    let res = Response::default().add_attribute("hub_addr", hub_addr.to_string());

    Ok(res)
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HubConfig {
    pub hub_addr: Addr,
}
