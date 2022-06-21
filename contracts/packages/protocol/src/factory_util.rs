use cosmwasm_std::{
    to_binary, Addr, ContractResult, Deps, QuerierWrapper, QueryRequest, SubMsgResponse, WasmQuery,
};

use crate::factory::{Config, QueryMsg};

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
