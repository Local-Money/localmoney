use crate::trade::TradeState;
use cosmwasm_std::{to_binary, Addr, QuerierWrapper, QueryRequest, StdResult, WasmQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    IncreaseTradeCount {
        profile_address: Addr,
        final_trade_state: TradeState,
    },
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Profile { address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

// Query Util
pub fn load_profile(
    querier: &QuerierWrapper,
    profile_address: Addr,
    profile_contract: String,
) -> Profile {
    let load_profile_result: StdResult<Profile> =
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: profile_contract,
            msg: to_binary(&QueryMsg::Profile {
                address: profile_address.clone(),
            })
            .unwrap(),
        }));

    load_profile_result.unwrap_or(Profile::new(profile_address))
}

// Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Profile {
    pub address: Addr,
    pub trade_count: u64,
}

impl Profile {
    pub const fn new(address: Addr) -> Self {
        Profile {
            address,
            trade_count: 0,
        }
    }
}
