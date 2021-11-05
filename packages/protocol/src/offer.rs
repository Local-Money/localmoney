use crate::currencies::FiatCurrency;
use crate::trade::State as TradeState;
use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

pub static CONFIG_KEY: &[u8] = b"config";
pub static OFFERS_KEY: &[u8] = b"offers";
pub const OFFERS: Map<&[u8], Offer> = Map::new("offers");

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferMsg {
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: u64,
    pub max_amount: u64, // TODO change to Uint128
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
        ust_amount: String,
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

impl Offer {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        OFFERS.save(storage, &self.id.to_be_bytes(), &self)
    }

    pub fn activate(&mut self, storage: &mut dyn Storage) -> StdResult<()> {
        self.state = OfferState::Active;
        self.save(storage)
    }

    pub fn pause(&mut self, storage: &mut dyn Storage) -> StdResult<()> {
        self.state = OfferState::Paused;
        self.save(storage)
    }

    pub fn update(&mut self, storage: &mut dyn Storage, msg: OfferMsg) -> StdResult<()> {
        self.offer_type = msg.offer_type;
        self.fiat_currency = msg.fiat_currency;
        self.min_amount = Uint128::from(msg.min_amount);
        self.max_amount = Uint128::from(msg.max_amount);
        self.save(storage)
    }
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
impl fmt::Display for OfferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferState {
    Active,
    Paused,
}
