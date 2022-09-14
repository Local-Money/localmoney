use super::constants::OFFERS_KEY;
use crate::currencies::FiatCurrency;
use crate::denom_utils::denom_to_string;
use crate::hub_utils::get_hub_config;
use crate::profile::load_profile;
use crate::trade::{Trade, TradeState};
use cosmwasm_std::{
    to_binary, Addr, Deps, Order, QuerierWrapper, QueryRequest, StdResult, Storage, Uint128,
    WasmQuery,
};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::ops::Add;

pub static CONFIG_KEY: &[u8] = b"config";

pub struct OfferIndexes<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, String, Offer, String>,
    pub filter: MultiIndex<'a, String, Offer, String>,
}

impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![&self.owner, &self.filter];
        Box::new(v.into_iter())
    }
}

pub fn offers<'a>() -> IndexedMap<'a, String, Offer, OfferIndexes<'a>> {
    let indexes = OfferIndexes {
        owner: MultiIndex::new(|d| d.owner.clone().to_string(), "offers", "offers__owner"),
        filter: MultiIndex::new(
            |offer: &Offer| {
                offer
                    .fiat_currency
                    .to_string()
                    .add(offer.offer_type.to_string().as_str())
                    .add(denom_to_string(&offer.denom).as_str())
                    .add(&offer.state.to_string())
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
    pub owner_contact: String,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub denom: Denom,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferUpdateMsg {
    pub id: String,
    pub owner_contact: Option<String>,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    //TODO: Change to Create(OfferMsg)
    Create { offer: OfferMsg },
    RegisterHub {},
    UpdateOffer { offer_update: OfferUpdateMsg },
    UpdateLastTraded { offer_id: String },
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
    State {},
    Offer {
        id: String,
    },
    Offers {
        owner: Option<Addr>,
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
    },
    OffersBy {
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        denom: Denom,
        min: Option<String>,
        max: Option<String>,
        order: QueryOrder,
        limit: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffersCount {
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: String,
    pub owner: Addr,
    pub owner_contact: String,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub denom: Denom,
    pub state: OfferState,
    pub timestamp: u64,
    pub trades_count: u64,
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
        if msg.owner_contact.is_some() {
            self.offer.owner_contact = msg.owner_contact.unwrap();
        }
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

    pub fn query(
        deps: Deps,
        owner: Option<Addr>,
        min: Option<String>,
        max: Option<String>,
        limit: u32,
        order: QueryOrder,
    ) -> StdResult<Vec<Offer>> {
        let hub_config = get_hub_config(deps);
        let storage = deps.storage;

        let std_order = match order {
            QueryOrder::Asc => Order::Ascending,
            QueryOrder::Desc => Order::Descending,
        };

        let range_min = match min {
            Some(thing) => Some(Bound::ExclusiveRaw(thing.into())),
            None => None,
        };

        let range_max = match max {
            Some(thing) => Some(Bound::ExclusiveRaw(thing.into())),
            None => None,
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
            .flat_map(|item| {
                item.and_then(|(_, mut offer)| {
                    let profile = load_profile(
                        &deps.querier,
                        offer.clone().owner,
                        hub_config.profile_addr.to_string(),
                    );
                    offer.trades_count = profile.trade_count;
                    Ok(offer)
                })
            })
            .collect();

        Ok(result)
    }

    pub fn query_by(
        deps: Deps,
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        denom: Denom,
        min: Option<String>,
        max: Option<String>,
        order: QueryOrder,
        limit: u32,
    ) -> StdResult<Vec<Offer>> {
        let hub_config = get_hub_config(deps);
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
            .prefix(
                fiat_currency.to_string()
                    + &offer_type.to_string()
                    + &denom_to_string(&denom)
                    + &*OfferState::Active.to_string(),
            )
            .range(storage, range_min, range_max, std_order)
            .take(limit as usize)
            .flat_map(|item| {
                item.and_then(|(_, mut offer)| {
                    let profile = load_profile(
                        &deps.querier,
                        offer.clone().owner,
                        hub_config.profile_addr.to_string(),
                    );
                    offer.trades_count = profile.trade_count;
                    Ok(offer)
                })
            })
            .collect();

        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeInfo {
    pub trade: Trade,
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
    pub fiat: FiatCurrency,
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

//Queries
pub fn load_offer(
    querier: &QuerierWrapper,
    offer_id: String,
    offer_contract: String,
) -> Option<Offer> {
    let load_offer_result: StdResult<Offer> =
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: offer_contract,
            msg: to_binary(&QueryMsg::Offer { id: offer_id }).unwrap(),
        }));

    if load_offer_result.is_err() {
        None
    } else {
        Some(load_offer_result.unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
