use super::constants::OFFERS_KEY;
use crate::currencies::FiatCurrency;
// use crate::errors::GuardError;
use crate::trade::{TradeData, TradeState};
use cosmwasm_std::{Addr, Deps, Order, StdResult, Storage, Uint128};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::ops::Add;

pub static CONFIG_KEY: &[u8] = b"config";
// pub const OFFERS: Map<&[u8], Offer> = Map::new(OFFERS_KEY);
pub struct OfferIndexes<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, String, Offer, String>,
    pub offer_type: MultiIndex<'a, String, Offer, String>,
    pub fiat: MultiIndex<'a, String, Offer, String>,
    pub filter: MultiIndex<'a, String, Offer, String>,
}

impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> =
            vec![&self.owner, &self.offer_type, &self.fiat, &self.filter];
        Box::new(v.into_iter())
    }
}

pub fn offers<'a>() -> IndexedMap<'a, String, Offer, OfferIndexes<'a>> {
    let indexes = OfferIndexes {
        owner: MultiIndex::new(|d| d.owner.clone().to_string(), "offers", "offers__owner"),
        offer_type: MultiIndex::new(
            |d: &Offer| d.offer_type.to_string(),
            "offers",
            "offers__offer_type",
        ),
        fiat: MultiIndex::new(
            |d: &Offer| d.fiat_currency.to_string(),
            "offers",
            "offers__fiat",
        ),
        filter: MultiIndex::new(
            |d: &Offer| {
                d.fiat_currency
                    .to_string()
                    .add(d.state.to_string().as_str())
                    .to_string()
            },
            "offers",
            "offers__filter",
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
    pub denom: Denom,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferUpdateMsg {
    pub id: String,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create {
        offer: OfferMsg,
    },
    RegisterHub,
    UpdateOffer {
        offer_update: OfferUpdateMsg,
    },
    NewTrade {
        offer_id: String,
        amount: Uint128,
        taker: Addr,
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
    UpdateLastTraded,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradesIndex {
    Seller,
    Buyer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryOrder {
    Asc,
    Desc,
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
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
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
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub offers_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: String,
    pub owner: Addr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub denom: Denom,
    pub state: OfferState,
    pub timestamp: u64,
    pub last_traded_at: u64,
}

pub struct OfferModel<'a> {
    pub offer: Offer,
    pub storage: &'a mut dyn Storage,
}

impl OfferModel<'_> {
    pub fn store(storage: &mut dyn Storage, offer: &Offer) -> StdResult<()> {
        offers().save(storage, offer.id.to_string(), &offer)
    }

    pub fn from_store(storage: &mut dyn Storage, id: &String) -> Offer {
        offers()
            .may_load(storage, id.to_string())
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

    pub fn update(&mut self, msg: OfferUpdateMsg) -> &Offer {
        self.offer.rate = msg.rate;
        self.offer.min_amount = msg.min_amount;
        self.offer.max_amount = msg.max_amount;
        self.offer.state = msg.state;
        OfferModel::store(self.storage, &self.offer).unwrap();
        &self.offer
        // self.save()
        //     ^^^^ move occurs because `*self` has type `OfferModel<'_>`, which does not implement the `Copy` trait
    }

    pub fn update_last_traded(&mut self, last_traded_at: u64) -> &Offer {
        self.offer.last_traded_at = last_traded_at;
        OfferModel::store(self.storage, &self.offer).unwrap();
        &self.offer
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
            Some(thing) => Some(Bound::exclusive(thing)),
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
        fiat_currency: FiatCurrency,
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;

        let std_order = match order {
            QueryOrder::Asc => Order::Ascending,
            QueryOrder::Desc => Order::Descending,
        };
        let range_min = match min {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let range_max = match max {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let result = offers()
            .idx
            .filter
            .prefix(fiat_currency.to_string() + &*OfferState::Active.to_string())
            .range(storage, range_min, range_max, std_order)
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
            Some(thing) => Some(Bound::exclusive(thing)),
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
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
    ) -> StdResult<Vec<Offer>> {
        let storage = deps.storage;
        // let range: Box<dyn Iterator<Item = StdResult<Pair<Offer>>>>;

        let range_min = match min {
            Some(thing) => Some(Bound::ExclusiveRaw(thing.into())),
            None => None,
        };

        let range_max = match max {
            Some(thing) => Some(Bound::ExclusiveRaw(thing.into())),
            None => None,
        };

        let std_order = match order {
            QueryOrder::Asc => Order::Ascending,
            QueryOrder::Desc => Order::Descending,
        };

        // Handle optional owner address query parameter
        let range = match owner {
            None => offers().range(storage, range_min, range_max, std_order),
            Some(unchecked_addr) => {
                let owner_addr = deps.api.addr_validate(unchecked_addr.as_str()).unwrap();

                offers()
                    .idx
                    .owner
                    .prefix(owner_addr.into_string())
                    .range(storage, range_min, range_max, std_order)
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
    pub trade: Addr,
    pub seller: Addr,
    pub buyer: Addr,
    pub arbitrator: Addr,
    pub state: TradeState,
    pub offer_id: String,
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
    Archive,
}
