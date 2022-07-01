use crate::factory::{Config, QueryMsg};
use cosmwasm_std::{
    to_binary, Addr, ContractResult, Deps, QuerierWrapper, QueryRequest, Response, Storage,
    SubMsgResponse, WasmQuery,
};
use cw_storage_plus::Item;
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

pub const HUB_CONFIG: Item<HubConfig> = Item::new("HubConfig");

pub fn register_hub_internal<E>(
    hub_addr: Addr,
    store: &mut dyn Storage,
    error: E,
) -> Result<Response, E> {
    let cfg = HUB_CONFIG.may_load(store).unwrap();
    if cfg.is_some() {
        return Err(error);
    }
    HUB_CONFIG
        .save(
            store,
            &HubConfig {
                hub_addr: hub_addr.clone(),
            },
        )
        .unwrap();
    let res = Response::new().add_attribute("hub_addr", hub_addr.to_string());

    Ok(res)
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HubConfig {
    pub hub_addr: Addr,
}
