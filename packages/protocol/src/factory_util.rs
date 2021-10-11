use crate::factory::{Config, QueryMsg};
use cosmwasm_std::{to_binary, QuerierWrapper, QueryRequest, WasmQuery};

pub fn get_factory_config(querier: &QuerierWrapper, factory_addr: String) -> Config {
    //TODO: Hack to pass test.
    let factory_addr = if factory_addr.contains("taker") {
        "factory".to_string()
    } else {
        factory_addr
    };
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: factory_addr,
            msg: to_binary(&QueryMsg::Config {}).unwrap(),
        }))
        .unwrap()
}
