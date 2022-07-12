use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterTrade { trade: String, maker: String },
    ClaimRewards { period: u8 },
    StartDistribution {},
    RegisterHub {},
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

//Data
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Distribution {
    pub start_time: u64,
    pub end_time: u64,
    pub period_duration: u64,
    pub periods: u8,
    pub current_period: u8,
    pub tokens_per_period: Uint128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TraderRewards {
    pub amount: Uint128,
}
