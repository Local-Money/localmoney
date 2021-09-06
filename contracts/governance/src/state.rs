use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub gov_token_addr: Addr,
    pub offers_addr: Addr,
    pub fee_collector_addr: Addr,
}

pub fn config_store(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_shares: Uint128,
}

pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Staker {
    pub address: String,
    pub shares: Uint128,
}

pub fn stakers_read<'a>(storage: &'a dyn Storage) -> ReadonlyBucket<'a, Uint128> {
    bucket_read(storage, b"stakers")
}

pub fn stakers_store<'a>(storage: &'a mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, b"stakers")
}
