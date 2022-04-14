use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use localterra_protocol::offer::{Arbitrator, Config, State, TradeAddr};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";

pub struct TradeIndexes<'a> {
    // pk goes to second tuple element
    pub seller: MultiIndex<'a, (Addr, Vec<u8>), TradeAddr>,
    pub buyer: MultiIndex<'a, (Addr, Vec<u8>), TradeAddr>,
    pub arbitrator: MultiIndex<'a, (Addr, Vec<u8>), TradeAddr>,
    pub arbitrator_state: MultiIndex<'a, (Addr, String, Vec<u8>), TradeAddr>,
}

impl<'a> IndexList<TradeAddr> for TradeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TradeAddr>> + '_> {
        let v: Vec<&dyn Index<TradeAddr>> = vec![
            &self.seller,
            &self.buyer,
            &self.arbitrator,
            &self.arbitrator_state,
        ];
        Box::new(v.into_iter())
    }
}

pub fn trades<'a>() -> IndexedMap<'a, &'a str, TradeAddr, TradeIndexes<'a>> {
    let indexes = TradeIndexes {
        seller: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.seller.clone(), k),
            "trades",         // TODO replace with TRADES_KEY
            "trades__seller", // TODO replace with TRADES_KEY and concat
        ),
        buyer: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.buyer.clone(), k),
            "trades",        // TODO replace with TRADES_KEY
            "trades__buyer", // TODO replace with TRADES_KEY and concat
        ),
        arbitrator: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.arbitrator.clone(), k),
            "trades",             // TODO replace with TRADES_KEY
            "trades__arbitrator", // TODO replace with TRADES_KEY and concat
        ),
        arbitrator_state: MultiIndex::new(
            |d: &TradeAddr, k: Vec<u8>| (d.arbitrator.clone(), d.state.clone().to_string(), k),
            "trades",                   // TODO replace with TRADES_KEY
            "trades__arbitrator_state", // TODO replace with TRADES_KEY and concat
        ),
    };
    IndexedMap::new("trades", indexes)
}

pub struct ArbitratorIndexes<'a> {
    // pk goes to second tuple element
    pub arbitrator: MultiIndex<'a, (Addr, Vec<u8>), Arbitrator>,
    pub asset: MultiIndex<'a, (String, Vec<u8>), Arbitrator>,
}

impl<'a> IndexList<Arbitrator> for ArbitratorIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Arbitrator>> + '_> {
        let v: Vec<&dyn Index<Arbitrator>> = vec![&self.arbitrator, &self.asset];
        Box::new(v.into_iter())
    }
}

pub fn arbitrators<'a>() -> IndexedMap<'a, &'a str, Arbitrator, ArbitratorIndexes<'a>> {
    let indexes = ArbitratorIndexes {
        arbitrator: MultiIndex::new(
            |d: &Arbitrator, k: Vec<u8>| (d.arbitrator.clone(), k),
            "arbitrators",             // TODO replace with arbitrators_KEY
            "arbitrators__arbitrator", // TODO replace with arbitrators_KEY and concat
        ),
        asset: MultiIndex::new(
            |d: &Arbitrator, k: Vec<u8>| (d.asset.clone().to_string(), k),
            "arbitrators",        // TODO replace with arbitrators_KEY
            "arbitrators__asset", // TODO replace with arbitrators_KEY and concat
        ),
    };
    IndexedMap::new("arbitrators", indexes)
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
