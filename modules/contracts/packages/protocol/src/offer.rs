use super::constants::OFFERS_KEY;
use crate::currencies::FiatCurrency;
use crate::errors::GuardError;
use crate::trade::{TradeData, TradeState};
use cosmwasm_std::{Addr, Deps, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

pub static CONFIG_KEY: &[u8] = b"config";
// pub const OFFERS: Map<&[u8], Offer> = Map::new(OFFERS_KEY);
pub struct OfferIndexes<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, (Addr, Vec<u8>), Offer>,
    pub offer_type: MultiIndex<'a, (String, Vec<u8>), Offer>,
    pub fiat: MultiIndex<'a, (String, Vec<u8>), Offer>,
    pub filter: MultiIndex<'a, (String, String, Vec<u8>), Offer>,
}

impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> =
            vec![&self.owner, &self.offer_type, &self.fiat, &self.filter];
        Box::new(v.into_iter())
    }
}

pub fn offers<'a>() -> IndexedMap<'a, &'a str, Offer, OfferIndexes<'a>> {
    let indexes = OfferIndexes {
        owner: MultiIndex::new(
            |d: &Offer, k: Vec<u8>| (d.owner.clone(), k),
            "offers",        // TODO replace with OFFERS_KEY
            "offers__owner", // TODO replace with OFFERS_KEY and concat
        ),
        offer_type: MultiIndex::new(
            |d: &Offer, k: Vec<u8>| (d.offer_type.to_string(), k),
            "offers",             // TODO replace with OFFERS_KEY
            "offers__offer_type", // TODO replace with OFFERS_KEY and concat
        ),
        fiat: MultiIndex::new(
            |d: &Offer, k: Vec<u8>| (d.fiat_currency.to_string(), k),
            "offers",       // TODO replace with OFFERS_KEY
            "offers__fiat", // TODO replace with OFFERS_KEY and concat
        ),
        filter: MultiIndex::new(
            |d: &Offer, k: Vec<u8>| {
                (
                    d.offer_type.to_string(),
                    d.fiat_currency.to_string() + &*d.state.to_string(),
                    k,
                )
            },
            "offers",         // TODO replace with OFFERS_KEY
            "offers__filter", // TODO replace with OFFERS_KEY and concat
        ),
    };
    IndexedMap::new(OFFERS_KEY, indexes)
}

// pub const OFFERS : IndexedMap<&str, Offer, OfferIndexes> = create_offers_indexedmap();

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferMsg {
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub maker_contact: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create {
        offer: OfferMsg,
    },
    Pause {
        id: String,
    },
    Activate {
        id: String,
    },
    Update {
        id: String,
        offer: OfferMsg,
    },
    NewTrade {
        offer_id: String,
        ust_amount: Uint128,
        taker: String, // TODO should be Addr
        taker_contact: String,
    },
    NewArbitrator {
        arbitrator: Addr,
        asset: FiatCurrency,
    },
    DeleteArbitrator {
        arbitrator: Addr,
        asset: FiatCurrency,
    },
    UpdateTradeArbitrator {
        arbitrator: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradesIndex {
    Seller,
    Buyer,
    ArbitratorState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    Offers {
        // TODO deprecated, remove
        fiat_currency: FiatCurrency,
    },
    OffersQuery {
        owner: Option<Addr>,
        last_value: Option<String>,
        limit: u32,
    },
    OffersByType {
        offer_type: OfferType,
        last_value: Option<String>,
        limit: u32,
    },
    OffersByFiat {
        fiat_currency: FiatCurrency,
        last_value: Option<String>,
        limit: u32,
    },
    OffersByTypeFiat {
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        last_value: Option<String>,
        limit: u32,
    },
    Offer {
        id: String,
    },
    TradesQuery {
        user: Addr,
        state: Option<TradeState>,
        index: TradesIndex,
        last_value: Option<Addr>,
        limit: u32,
    },
    Arbitrator {
        arbitrator: Addr,
    },
    Arbitrators {
        last_value: Option<String>,
        limit: u32,
    },
    ArbitratorAsset {
        asset: FiatCurrency,
    },
    ArbitratorRandom {
        random_value: u32,
        asset: FiatCurrency,
    },
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub factory_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub offers_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: String,
    pub owner: Addr,
    pub maker_contact: String,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
    pub timestamp: u64,
}

pub struct OfferModel<'a> {
    pub offer: Offer,
    pub storage: &'a mut dyn Storage,
}

impl OfferModel<'_> {
    pub fn store(storage: &mut dyn Storage, offer: &Offer) -> StdResult<()> {
        offers().save(storage, &offer.id, &offer)
    }

    pub fn from_store(storage: &mut dyn Storage, id: &String) -> Offer {
        offers()
            .may_load(storage, &id.to_string())
            .unwrap_or_default()
            .unwrap()
    }

    pub fn create(storage: &mut dyn Storage, offer: Offer) -> OfferModel {
        OfferModel::store(storage, &offer).unwrap();
        OfferModel { offer, storage }
    }

    pub fn save<'a>(self) -> Offer {
        OfferModel::store(self.storage, &self.offer).unwrap();
        self.offer
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: &String) -> OfferModel<'a> {
        let offer_model = OfferModel {
            offer: OfferModel::from_store(storage, &id),
            storage,
        };
        return offer_model;
    }

    pub fn activate(&mut self) -> Result<&Offer, GuardError> {
        match self.offer.state {
            OfferState::Paused => {
                self.offer.state = OfferState::Active;
                OfferModel::store(self.storage, &self.offer).unwrap();
                Ok(&self.offer)
            }
            OfferState::Active => Err(GuardError::InvalidStateChange {
                from: self.offer.state.clone(),
                to: OfferState::Active,
            }),
        }
    }

    pub fn pause(&mut self) -> Result<&Offer, GuardError> {
        match self.offer.state {
            OfferState::Active => {
                self.offer.state = OfferState::Paused;
                OfferModel::store(self.storage, &self.offer).unwrap();
                Ok(&self.offer)
            }
            OfferState::Paused => Err(GuardError::InvalidStateChange {
                from: self.offer.state.clone(),
                to: OfferState::Paused,
            }),
        }
    }

    pub fn update(&mut self, msg: OfferMsg) -> &Offer {
        self.offer.offer_type = msg.offer_type;
        self.offer.fiat_currency = msg.fiat_currency;
        self.offer.rate = msg.rate;
        self.offer.min_amount = msg.min_amount;
        self.offer.max_amount = msg.max_amount;
        OfferModel::store(self.storage, &self.offer).unwrap();
        &self.offer
        // self.save()
        //     ^^^^ move occurs because `*self` has type `OfferModel<'_>`, which does not implement the `Copy` trait
    }

    pub fn query_all_offers(
        storage: &dyn Storage,
        fiat_currency: FiatCurrency,
    ) -> StdResult<Vec<Offer>> {
        let result: Vec<Offer> = offers()
            .range(storage, None, None, Order::Descending)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
            .filter(|offer| offer.fiat_currency == fiat_currency)
            .collect();

        Ok(result)
    }

    pub fn query_by_type(
        deps: Deps,
        offer_type: OfferType,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;

        let range_from = match last_value {
            Some(thing) => Some(Bound::Exclusive(Vec::from(thing.to_string()))),
            None => None,
        };

        let result = offers()
            .idx
            .offer_type
            .prefix(offer_type.to_string())
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
            .collect();

        Ok(result)
    }

    pub fn query_by_type_fiat(
        deps: Deps,
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;

        let range_from = match last_value {
            Some(thing) => Some(Bound::Exclusive(Vec::from(thing.to_string()))),
            None => None,
        };

        let result = offers()
            .idx
            .filter
            .prefix((
                offer_type.to_string(),
                fiat_currency.to_string() + &*OfferState::Active.to_string(),
            ))
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
            .collect();

        Ok(result)
    }

    pub fn query_by_fiat(
        deps: Deps,
        fiat_currency: FiatCurrency,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;

        let range_from = match last_value {
            Some(thing) => Some(Bound::Exclusive(Vec::from(thing.to_string()))),
            None => None,
        };

        let result = offers()
            .idx
            .fiat
            .prefix(fiat_currency.to_string())
            .range(storage, range_from, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
            .collect();

        Ok(result)
    }

    pub fn query(
        deps: Deps,
        owner: Option<Addr>,
        last_value: Option<String>,
        limit: u32,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;
        // let range: Box<dyn Iterator<Item = StdResult<Pair<Offer>>>>;

        let range_from = match last_value {
            Some(thing) => Some(Bound::Exclusive(Vec::from(thing))),
            None => None,
        };

        // Handle optional owner address query parameter
        let range = match owner {
            None => offers().range(storage, range_from, None, Order::Descending),
            Some(unchecked_addr) => {
                let owner_addr = deps.api.addr_validate(unchecked_addr.as_str()).unwrap();

                offers().idx.owner.prefix(owner_addr).range(
                    storage,
                    range_from,
                    None,
                    Order::Descending,
                )
            }
        };

        let result = range
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
            .collect();

        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeInfo {
    pub trade: TradeData,
    pub offer: Offer,
    pub expired: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeAddr {
    pub trade: Addr, // TODO rename to tradeAddr
    pub seller: Addr,
    pub buyer: Addr,
    pub arbitrator: Addr,
    pub state: TradeState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Arbitrator {
    pub arbitrator: Addr,
    pub asset: FiatCurrency,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    Buy,
    Sell,
}

impl fmt::Display for OfferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for OfferState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferState {
    Active,
    Paused,
}
