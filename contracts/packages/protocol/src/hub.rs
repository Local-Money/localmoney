use cosmwasm_std::{Addr, Decimal};
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
    pub price_addr: Addr,
    pub price_provider_addr: Addr,
    pub local_market_addr: Addr,
    pub local_denom: Denom,
    pub chain_fee_collector_addr: Addr,
    pub warchest_addr: Addr,
    pub active_offers_limit: u8,
    pub active_trades_limit: u8,
    pub arbitration_fee_pct: Decimal,
    pub burn_fee_pct: Decimal,
    pub chain_fee_pct: Decimal,
    pub warchest_fee_pct: Decimal,
    pub trade_expiration_timer: u64, // in seconds
    pub trade_dispute_timer: u64,
    pub trade_limit_min: u128, // in USD
    pub trade_limit_max: u128, // in USD
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
