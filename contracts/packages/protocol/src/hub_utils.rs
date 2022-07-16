use crate::hub::{HubConfig, QueryMsg};
use cosmwasm_std::{to_binary, Addr, QuerierWrapper, QueryRequest, Response, Storage, WasmQuery};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_hub_config(querier: &QuerierWrapper, hub_addr: String) -> HubConfig {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: hub_addr,
            msg: to_binary(&QueryMsg::Config {}).unwrap(),
        }))
        .unwrap()
}

pub const HUB_ADDR: Item<HubAddr> = Item::new("HubConfig");

pub fn register_hub_internal<E>(
    hub_addr: Addr,
    store: &mut dyn Storage,
    error: E,
) -> Result<Response, E> {
    let cfg = HUB_ADDR.may_load(store).unwrap();
    if cfg.is_some() {
        return Err(error);
    }
    HUB_ADDR
        .save(
            store,
            &HubAddr {
                addr: hub_addr.clone(),
            },
        )
        .unwrap();
    let res = Response::new().add_attribute("hub_addr", hub_addr.to_string());

    Ok(res)
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HubAddr {
    pub addr: Addr,
}
