use crate::currencies::FiatCurrency;
use crate::trade::State as TradeState;
use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static CONFIG_KEY: &[u8] = b"config";
pub static OFFERS_KEY: &[u8] = b"offers";

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

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
    Create {
        offer: OfferMsg,
    },
    Pause {
        id: u64,
    },
    Activate {
        id: u64,
    },
    Update {
        id: u64,
        offer: OfferMsg,
    },
    NewTrade {
        offer_id: u64,
        ust_amount: Uint128,
        counterparty: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    Offers { fiat_currency: FiatCurrency },
    Offer { id: u64 },
    Trades { maker: String },
    TradeInfo { maker: String, trade: String },
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub factory_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub offers_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub owner: Addr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeInfo {
    pub trade: TradeState,
    pub offer: Offer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferState {
    Active,
    Paused,
}
