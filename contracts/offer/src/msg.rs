use crate::currencies::FiatCurrency;
use crate::state::OfferType;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub gov_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferMsg {
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: u64,
    pub max_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create { offer: OfferMsg },
    Pause { id: u64 },
    Activate { id: u64 },
    Update { id: u64, offer: OfferMsg },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    LoadOffers { fiat_currency: FiatCurrency },
    LoadOffer { id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub offers_count: u64,
}

/// Governance models
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceQueryMsg {
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GovernanceConfigResponse {
    pub gov_token_addr: Addr,
    pub offers_addr: Addr,
    pub fee_collector_addr: Addr,
}
