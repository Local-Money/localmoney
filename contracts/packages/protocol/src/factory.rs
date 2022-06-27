use cosmwasm_std::Addr;
use cw20::Denom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin_addr: Addr,
    pub trading_incentives_code_id: u64,
    pub offer_code_id: u64,
    pub trade_code_id: u64,
    pub local_denom: Denom,
    pub local_market_addr: Addr,
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
    pub admin_addr: Addr,
    pub trade_code_id: u64,
    pub local_denom: Denom,
    pub local_market_addr: Addr,
    pub offers_addr: Addr,
    pub trading_incentives_addr: Addr,
    pub warchest_addr: Option<Addr>,
}
