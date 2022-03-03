use cosmwasm_std::{Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use localterra_protocol::governance::{Config, State};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";

pub fn config_store(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

pub fn stakers_read<'a>(storage: &'a dyn Storage) -> ReadonlyBucket<'a, Uint128> {
    bucket_read(storage, b"stakers")
}

pub fn stakers_store<'a>(storage: &'a mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, b"stakers")
}
