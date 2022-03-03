use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_code_id: u64,
    pub gov_contract_code_id: u64,
    pub fee_collector_code_id: u64,
    pub trading_incentives_code_id: u64,
    pub offer_code_id: u64,
    pub trade_code_id: u64,
    pub fee_collector_threshold: Uint128,
    pub local_ust_pool_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub trade_code_id: u64,
    pub token_addr: Addr,
    pub local_ust_pool_addr: Addr,
    pub gov_addr: Addr,
    pub offers_addr: Addr,
    pub fee_collector_addr: Addr,
    pub trading_incentives_addr: Addr,
}
