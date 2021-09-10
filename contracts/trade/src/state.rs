use cosmwasm_std::{Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use localterra_protocol::trade::{State, CONFIG_KEY};

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
