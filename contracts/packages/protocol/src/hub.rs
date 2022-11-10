use cosmwasm_std::Addr;
use cw20::Denom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig(HubConfig),
    UpdateAdmin { admin_addr: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Admin {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Admin {
    pub addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HubConfig {
    pub offer_addr: Addr,
    pub trade_addr: Addr,
    pub profile_addr: Addr,
    pub trading_incentives_addr: Addr,
    pub price_addr: Addr,
    pub local_market_addr: Addr,
    pub local_denom: Denom,
    pub chain_fee_collector_addr: Addr,
    pub warchest_addr: Addr,
    pub burn_fee_pct: u128,
    pub chain_fee_pct: u128,
    pub warchest_fee_pct: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
