use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Env, HumanAddr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub recipient: HumanAddr,
    pub sender: HumanAddr,
    pub offer_id: u64,
    pub state: TradeState,
    pub expire_height: u64,
    pub amount: Uint128,
}

//TODO: How to query offer data without copying all the types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub owner: HumanAddr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OrderState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FiatCurrency {
    Cop,
    Brl,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Active,
    Paused,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradeState {
    Canceled,
    Closed,
    Created,
    EscrowFunded,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
