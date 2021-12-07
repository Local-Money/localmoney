use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use localterra_protocol::offer::{Config, State, TradeAddr};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub sender: MultiIndex<'a, (Addr, Vec<u8>), TradeAddr>,
    pub recipient: MultiIndex<'a, (Addr, Vec<u8>), TradeAddr>,
}

impl<'a> IndexList<TradeAddr> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TradeAddr>> + '_> {
        let v: Vec<&dyn Index<TradeAddr>> = vec![&self.sender, &self.recipient];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, &'a str, TradeAddr, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        sender: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.sender.clone(), k),
            "trades",         // TODO replace with TRADES_KEY
            "trades__sender", // TODO replace with TRADES_KEY and concat
        ),
        recipient: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.recipient.clone(), k),
            "trades",            // TODO replace with TRADES_KEY
            "trades__recipient", // TODO replace with TRADES_KEY and concat
        ),
    };
    IndexedMap::new("trades", indexes)
}

pub fn config_storage(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state_storage(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}
