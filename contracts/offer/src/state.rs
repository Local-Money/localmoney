use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cosmwasm_storage::{
    bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};
use cw_storage_plus::{Index, IndexList, IndexedMap, Map, MultiIndex};
use localterra_protocol::constants::OFFERS_KEY;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{offers, Config, Offer, State, TradeInfo};
use localterra_protocol::trade::State as Trade;

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";
pub const TRADES: Map<&[u8], Vec<Addr>> = Map::new("trades");

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub sender: MultiIndex<'a, (Addr, Vec<u8>), Trade>,
    pub recipient: MultiIndex<'a, (Addr, Vec<u8>), Trade>,
}

impl<'a> IndexList<Trade> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Trade>> + '_> {
        let v: Vec<&dyn Index<Trade>> = vec![&self.sender, &self.recipient];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, &'a str, Trade, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        sender: MultiIndex::new(
            |d: &Trade, k: Vec<u8>| (d.sender.clone(), k),
            "trades",         // TODO replace with TRADES_KEY
            "trades__sender", // TODO replace with TRADES_KEY and concat
        ),
        recipient: MultiIndex::new(
            |d: &Trade, k: Vec<u8>| (d.recipient.clone(), k),
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

pub fn query_all_trades(storage: &dyn Storage, maker: Addr) -> StdResult<Vec<Addr>> {
    let result = TRADES.load(storage, maker.as_bytes());
    Ok(result.unwrap_or(vec![]))
}
