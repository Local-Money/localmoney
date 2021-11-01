use cosmwasm_std::{to_binary, QuerierWrapper, QueryRequest, WasmQuery};

use crate::factory::{Config, QueryMsg};

pub fn get_factory_config(querier: &QuerierWrapper, factory_addr: String) -> Config {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: factory_addr,
            msg: to_binary(&QueryMsg::Config {}).unwrap(),
        }))
        .unwrap()
}
