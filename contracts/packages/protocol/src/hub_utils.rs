use crate::hub::{Admin, HubConfig, QueryMsg};
use cosmwasm_std::{
    to_binary, Addr, CustomQuery, Deps, QueryRequest, Response, Storage, WasmQuery,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_hub_config<T: CustomQuery>(deps: Deps<T>) -> HubConfig {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: hub_addr.addr.to_string(),
            msg: to_binary(&QueryMsg::Config {}).unwrap(),
        }))
        .unwrap()
}

pub fn get_hub_admin<T: CustomQuery>(deps: Deps<T>) -> Admin {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: hub_addr.addr.to_string(),
            msg: to_binary(&QueryMsg::Admin {}).unwrap(),
        }))
        .unwrap()
}

pub const HUB_ADDR: Item<HubAddr> = Item::new("HubConfig");

pub fn register_hub_internal<T, E>(
    hub_addr: Addr,
    store: &mut dyn Storage,
    error: E,
) -> Result<Response<T>, E> {
    let cfg = HUB_ADDR.may_load(store).unwrap();
    if cfg.is_some() {
        return if cfg.unwrap().addr.ne(&hub_addr) {
            Err(error)
        } else {
            Ok(Response::default())
        };
    }
    HUB_ADDR
        .save(
            store,
            &HubAddr {
                addr: hub_addr.clone(),
            },
        )
        .unwrap();
    let res = Response::new()
        .add_attribute("action", "register_hub")
        .add_attribute("hub_addr", hub_addr.to_string());

    Ok(res)
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HubAddr {
    pub addr: Addr,
}
