use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use localterra_protocol::factory::Config;
use localterra_protocol::factory_util::HubConfig;
use localterra_protocol::offer::{Arbitrator, State, TradeAddr};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";

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

pub struct ArbitratorIndexes<'a> {
    // pk goes to second tuple element
    pub arbitrator: MultiIndex<'a, Addr, Arbitrator, Vec<u8>>,
    pub asset: MultiIndex<'a, String, Arbitrator, Vec<u8>>,
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
            |d: &Arbitrator| d.arbitrator.clone(),
            "arbitrators",
            "arbitrators__arbitrator",
        ),
        asset: MultiIndex::new(
            |d: &Arbitrator| d.asset.clone().to_string(),
            "arbitrators",
            "arbitrators__asset",
        ),
    };
    IndexedMap::new("arbitrators", indexes)
}

pub fn hub_config_storage(storage: &mut dyn Storage) -> Singleton<HubConfig> {
    singleton(storage, CONFIG_KEY)
}

pub fn hub_config_read(storage: &dyn Storage) -> ReadonlySingleton<HubConfig> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state_storage(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}
