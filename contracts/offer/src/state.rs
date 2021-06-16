use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::currencies::FiatCurrency;
use cosmwasm_std::{Deps, HumanAddr, Order, StdResult, Storage, Uint128};
use cosmwasm_storage::{bucket_read, singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static OFFERS_KEY: &[u8] = b"offers";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub offers_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub owner: HumanAddr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
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

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn query_all_offers(deps: Deps, fiat_currency: FiatCurrency) -> StdResult<Vec<Offer>> {
    let offers: Vec<Offer> = bucket_read(deps.storage, OFFERS_KEY)
        .range(None, None, Order::Descending)
        .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
        .collect();

    let result: Vec<Offer> = offers
        .iter()
        .filter(|offer| offer.fiat_currency == fiat_currency)
        .cloned()
        .collect();

    Ok(result)
}
