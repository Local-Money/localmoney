use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};

use localmoney_protocol::offer::{OffersCount, TradeAddr};

pub static OFFERS_COUNT_KEY: &[u8] = b"offers_count_v0_4_1";

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub seller: MultiIndex<'a, Addr, TradeAddr, Vec<u8>>,
    pub buyer: MultiIndex<'a, Addr, TradeAddr, Vec<u8>>,
    pub arbitrator: MultiIndex<'a, Addr, TradeAddr, Vec<u8>>,
}

impl<'a> IndexList<TradeAddr> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TradeAddr>> + '_> {
        let v: Vec<&dyn Index<TradeAddr>> = vec![&self.seller, &self.buyer, &self.arbitrator];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, &'a str, TradeAddr, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        seller: MultiIndex::new(|d: &TradeAddr| d.seller.clone(), "trades", "trades__seller"),
        buyer: MultiIndex::new(|d: &TradeAddr| d.buyer.clone(), "trades", "trades__buyer"),
        arbitrator: MultiIndex::new(
            |d: &TradeAddr| d.arbitrator.clone(),
            "trades",
            "trades__arbitrator",
        ),
    };
    IndexedMap::new("trades", indexes)
}

pub fn offers_count_storage(storage: &mut dyn Storage) -> Singleton<OffersCount> {
    singleton(storage, OFFERS_COUNT_KEY)
}

pub fn offers_count_read(storage: &dyn Storage) -> ReadonlySingleton<OffersCount> {
    singleton_read(storage, OFFERS_COUNT_KEY)
}
