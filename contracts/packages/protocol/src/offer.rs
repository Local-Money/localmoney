use crate::currencies::FiatCurrency;
use crate::denom_utils::denom_to_string;
use crate::guards::validate_min_max_items_per_page;
use crate::hub_utils::get_hub_config;
use crate::profile::{load_profile, load_profiles, Profile};
use crate::trade::{TradeResponse, TradeState};
use cosmwasm_std::{Addr, CustomQuery, Deps, Order, QuerierWrapper, StdResult, Storage, Uint128};
use cw20::Denom;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::ops::Add;

pub static CONFIG_KEY: &[u8] = b"config";

pub struct OfferIndexes<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, Addr, Offer, u64>,
    pub filter: MultiIndex<'a, String, Offer, u64>,
}

impl<'a> IndexList<Offer> for OfferIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offer>> + '_> {
        let v: Vec<&dyn Index<Offer>> = vec![&self.owner, &self.filter];
        Box::new(v.into_iter())
    }
}

pub fn offers<'a>() -> IndexedMap<'a, u64, Offer, OfferIndexes<'a>> {
    let offers_pk_namespace = "offers";
    let indexes = OfferIndexes {
        owner: MultiIndex::new(|d| d.owner.clone(), offers_pk_namespace, "offers__owner"),
        filter: MultiIndex::new(
            |offer: &Offer| {
                offer
                    .fiat_currency
                    .to_string()
                    .add(offer.offer_type.to_string().as_str())
                    .add(denom_to_string(&offer.denom).as_str())
                    .add(&offer.state.to_string())
            },
            offers_pk_namespace,
            "offers__filter",
        ),
    };
    IndexedMap::new(offers_pk_namespace, indexes)
}

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferMsg {
    pub offer_type: OfferType,
    pub owner_contact: String,
    pub owner_encryption_key: String,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub denom: Denom,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferUpdateMsg {
    pub id: u64,
    pub owner_contact: Option<String>,
    pub owner_encryption_key: Option<String>,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    //TODO: Change to Create(OfferMsg)
    Create { offer: OfferMsg },
    UpdateOffer { offer_update: OfferUpdateMsg },
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    State {},
    Offer {
        id: u64,
    },
    OffersBy {
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        denom: Denom,
        order: OfferOrder,
        limit: u32,
        last: Option<u64>,
    },
    OffersByOwner {
        owner: Addr,
        limit: u32,
        last: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffersCount {
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub owner: Addr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub rate: Uint128,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub description: Option<String>,
    pub denom: Denom,
    pub state: OfferState,
    pub timestamp: u64,
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
        offers().save(storage, offer.id, &offer)
    }

    pub fn from_store(storage: &mut dyn Storage, id: u64) -> Offer {
        offers().may_load(storage, id).unwrap_or_default().unwrap()
    }

    pub fn create(storage: &mut dyn Storage, offer: Offer) -> OfferModel {
        OfferModel::store(storage, &offer).unwrap();
        OfferModel { offer, storage }
    }

    pub fn save<'a>(self) -> Offer {
        OfferModel::store(self.storage, &self.offer).unwrap();
        self.offer
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: u64) -> OfferModel<'a> {
        let offer_model = OfferModel {
            offer: OfferModel::from_store(storage, id),
            storage,
        };
        return offer_model;
    }

    pub fn update(&mut self, msg: OfferUpdateMsg) -> &Offer {
        self.offer.rate = msg.rate;
        self.offer.min_amount = msg.min_amount;
        self.offer.max_amount = msg.max_amount;
        self.offer.state = msg.state;
        self.offer.description = msg.description;
        OfferModel::store(self.storage, &self.offer).unwrap();
        &self.offer
    }

    pub fn query_by_owner(
        deps: Deps,
        owner: Addr,
        limit: u32,
        last: Option<u64>,
    ) -> StdResult<Vec<OfferResponse>> {
        let hub_config = get_hub_config(deps);
        let range_from = last.map(Bound::exclusive);
        let limit = validate_min_max_items_per_page(limit);

        let result = offers()
            .idx
            .owner
            .prefix(owner)
            .range(deps.storage, None, range_from, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| {
                item.and_then(|(_, offer)| {
                    let profile = load_profile(
                        &deps.querier,
                        hub_config.profile_addr.to_string(),
                        offer.clone().owner,
                    )
                    .unwrap();
                    Ok(OfferResponse { offer, profile })
                })
            })
            .collect();

        Ok(result)
    }

    pub fn query_by<T: CustomQuery>(
        deps: Deps<T>,
        offer_type: OfferType,
        fiat_currency: FiatCurrency,
        denom: Denom,
        order: OfferOrder,
        limit: u32,
        last: Option<u64>,
    ) -> StdResult<Vec<OfferResponse>> {
        let hub_config = get_hub_config(deps);
        let storage = deps.storage;
        let std_order = Order::Descending;
        let range_from = last.map(Bound::exclusive);
        let limit = validate_min_max_items_per_page(limit);

        let mut profiles = load_profiles(
            &deps.querier,
            hub_config.profile_addr.to_string(),
            limit,
            None,
        )
        .unwrap();

        let prefix = fiat_currency.to_string()
            + &offer_type.to_string()
            + &denom_to_string(&denom)
            + &*OfferState::Active.to_string();

        let mut result: Vec<OfferResponse> = offers()
            .idx
            .filter
            .prefix(prefix)
            .range(storage, None, range_from, std_order)
            .flat_map(|item| {
                item.and_then(|(_, offer)| {
                    let profile_found = profiles
                        .clone()
                        .into_iter()
                        .find(|profile| profile.addr.eq(&offer.owner));

                    let profile = if profile_found.is_some() {
                        profile_found.unwrap()
                    } else {
                        let new_profile = load_profile(
                            &deps.querier,
                            hub_config.profile_addr.to_string(),
                            offer.owner.clone(),
                        )
                        .unwrap();
                        profiles.push(new_profile.clone());
                        new_profile
                    };

                    Ok(OfferResponse { offer, profile })
                })
            })
            .take(limit as usize)
            .collect();

        match order {
            OfferOrder::TradesCount => {
                result.sort_by(|prev, next| {
                    next.profile
                        .released_trades_count
                        .cmp(&prev.profile.released_trades_count)
                });
            }
            OfferOrder::PriceRate => {
                result.sort_by(|prev, next| prev.offer.rate.cmp(&next.offer.rate));
            }
        }

        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeInfo {
    pub trade: TradeResponse,
    pub offer: OfferResponse,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferOrder {
    TradesCount,
    PriceRate,
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

// Price

// Queries
pub fn load_offer<T: CustomQuery>(
    querier: &QuerierWrapper<T>,
    offer_id: u64,
    offer_contract: String,
) -> StdResult<OfferResponse> {
    querier.query_wasm_smart(offer_contract, &QueryMsg::Offer { id: offer_id })
}
// Migration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
