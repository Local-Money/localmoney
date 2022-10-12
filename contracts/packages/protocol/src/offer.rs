use super::constants::OFFERS_KEY;
use crate::constants::OFFERS_QUERY_OFFERS_BY_PROFILE;
use crate::constants::OFFERS_QUERY_PROFILES_LIMIT;
use crate::currencies::FiatCurrency;
use crate::denom_utils::denom_to_string;
use crate::hub_utils::get_hub_config;
use crate::profile::load_profile;
use crate::profile::Profile;
use crate::profile::QueryMsg as ProfileQueryMsg;
use crate::trade::{Trade, TradeState};
use cosmwasm_std::{Addr, Deps, Order, QuerierWrapper, StdResult, Storage, Uint128};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
pub enum QueryMsg {
    State {},
    Offer {
        id: String,
    },
    Offers {
        start_at: Option<u64>,
    },
    OffersBy {
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        denom: Denom,
        min: Option<String>,
        max: Option<String>,
        limit: u32,
    },
    OffersByOwner {
        owner: Addr,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub offer: Offer,
    pub profile: Profile,
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
    }

    pub fn update_last_traded(&mut self, last_traded_at: u64) -> &Offer {
        self.offer.last_traded_at = last_traded_at;
        self.offer.trades_count += 1;
        OfferModel::store(self.storage, &self.offer).unwrap();
        &self.offer
    }

    pub fn query(deps: Deps, start_at: Option<u64>) -> StdResult<Vec<OfferResponse>> {
        let hub_config = get_hub_config(deps);

        let profiles: Vec<Profile> = deps
            .querier
            .query_wasm_smart(
                hub_config.profile_addr.to_string(),
                &ProfileQueryMsg::Profiles {
                    limit: OFFERS_QUERY_PROFILES_LIMIT,
                    start_at,
                },
            )
            .unwrap_or(vec![]);

        let mut result: Vec<OfferResponse> = vec![];
        let offers_by_profile_limit = OFFERS_QUERY_OFFERS_BY_PROFILE;
        profiles.iter().for_each(|profile| {
            let offers_by_owner: Vec<Offer> =
                OfferModel::query_by_owner(deps, profile.addr.clone(), offers_by_profile_limit)
                    .unwrap_or(vec![]);

            offers_by_owner.iter().for_each(|offer| {
                result.push(OfferResponse {
                    offer: offer.clone(),
                    profile: profile.clone(),
                })
            });
        });
        Ok(result)
    }

    pub fn query_by_owner(deps: Deps, owner: Addr, limit: u32) -> StdResult<Vec<Offer>> {
        let range = offers().idx.owner.prefix(owner.into_string()).range(
            deps.storage,
            None,
            None,
            Order::Descending,
        );

        let result = range
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
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
        limit: u32,
    ) -> StdResult<Vec<OfferResponse>> {
        let hub_config = get_hub_config(deps);
        let storage = deps.storage;
        let std_order = Order::Descending;
        let range_min = match min {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let range_max = match max {
            Some(thing) => Some(Bound::exclusive(thing)),
            None => None,
        };

        let mut profiles: HashMap<Addr, Profile> = HashMap::new();
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
                item.and_then(|(_, offer)| {
                    let profile: Profile;
                    if profiles.contains_key(&offer.owner) {
                        profile = profiles.get(&offer.owner).unwrap().clone();
                    } else {
                        let profile_result = load_profile(
                            &deps.querier,
                            offer.clone().owner,
                            hub_config.profile_addr.to_string(),
                        );
                        profile = profile_result.unwrap().clone();
                        profiles.insert(offer.owner.clone(), profile.clone());
                    }
                    Ok(OfferResponse { offer, profile })
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

// Queries
pub fn load_offer(
    querier: &QuerierWrapper,
    offer_id: String,
    offer_contract: String,
) -> StdResult<OfferResponse> {
    querier.query_wasm_smart(offer_contract, &QueryMsg::Offer { id: offer_id })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
