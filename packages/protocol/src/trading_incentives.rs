use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterTrade { trade: String, maker: String },
    Claim { period: u8 },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    StartDistribution {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Distribution {},
    Rewards { trader: String, period: u8 },
}

///Data
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub factory_addr: Addr,
    pub distribution_start: u64,
    pub distribution_period_duration: u64,
    pub distribution_periods: u8,
    pub tokens_per_period: Uint128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Distribution {
    pub distribution_start_time: u64,
    pub distribution_end_time: u64,
    pub period_duration: u64,
    pub current_period: u8,
    pub tokens_per_period: Uint128,
}
