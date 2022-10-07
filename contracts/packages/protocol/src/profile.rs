use std::ops::Sub;

use crate::{constants::PROFILE_QUERY_LAST_TRADE_THRESHOLD, trade::TradeState};
use cosmwasm_std::{Addr, Deps, Env, Order, QuerierWrapper, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex, PrefixBound};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create {
        addr: Addr,
    },
    IncreaseTradeCount {
        profile_address: Addr,
        final_trade_state: TradeState,
    },
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Profile { address: Addr },
    Profiles { limit: u32, start_at: Option<u64> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

// Query Util
pub fn load_profile(
    querier: &QuerierWrapper,
    profile_addr: Addr,
    profile_contract: String,
) -> Profile {
    querier
        .query_wasm_smart(
            profile_contract,
            &QueryMsg::Profile {
                address: profile_addr.clone(),
            },
        )
        .unwrap_or(Profile::new(profile_addr, 0))
}

// Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Profile {
    pub addr: Addr,
    pub created_at: u64,
    pub trades_count: u64,
    pub last_trade: u64,
}

impl Profile {
    pub const fn new(addr: Addr, created_at: u64) -> Self {
        Profile {
            addr,
            created_at,
            trades_count: 0,
            last_trade: 0,
        }
    }
}

pub struct ProfileModel<'a> {
    pub profile: Profile,
    pub storage: &'a mut dyn Storage,
}

impl ProfileModel<'_> {
    pub fn store(storage: &mut dyn Storage, profile: &Profile) -> StdResult<()> {
        profiles().save(storage, profile.addr.to_string(), &profile)
    }

    pub fn from_store(storage: &mut dyn Storage, address: &String) -> Profile {
        profiles()
            .may_load(storage, address.clone())
            .unwrap_or_default()
            .unwrap()
    }

    pub fn create(storage: &mut dyn Storage, profile: Profile) -> ProfileModel {
        ProfileModel::store(storage, &profile).unwrap();
        ProfileModel { profile, storage }
    }

    pub fn save<'a>(self) -> Profile {
        ProfileModel::store(self.storage, &self.profile).unwrap();
        self.profile
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, address: &String) -> ProfileModel<'a> {
        ProfileModel {
            profile: ProfileModel::from_store(storage, address),
            storage,
        }
    }

    pub fn query(
        deps: Deps,
        env: Env,
        limit: u32,
        start_at: Option<u64>,
    ) -> StdResult<Vec<Profile>> {
        let min_range = match start_at {
            Some(time) => Some(PrefixBound::exclusive(time)),
            None => None,
        };
        let result = profiles()
            .idx
            .last_trade
            .prefix_range(deps.storage, min_range, None, Order::Descending)
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, profile)| Ok(profile)))
            .collect();
        Ok(result)
    }
}

pub struct ProfileIndexes<'a> {
    pub address: MultiIndex<'a, String, Profile, String>,
    pub trades_count: MultiIndex<'a, u64, Profile, String>,
    pub last_trade: MultiIndex<'a, u64, Profile, String>,
}

impl<'a> IndexList<Profile> for ProfileIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Profile>> + '_> {
        let v: Vec<&dyn Index<Profile>> = vec![&self.address, &self.trades_count, &self.last_trade];
        Box::new(v.into_iter())
    }
}

pub fn profiles<'a>() -> IndexedMap<'a, String, Profile, ProfileIndexes<'a>> {
    let indexes = ProfileIndexes {
        address: MultiIndex::new(
            |p: &Profile| p.addr.to_string(),
            "profiles",
            "profiles__address",
        ),
        trades_count: MultiIndex::new(
            |p: &Profile| p.trades_count,
            "profiles",
            "profiles__trades_count",
        ),
        last_trade: MultiIndex::new(
            |p: &Profile| p.last_trade,
            "profiles",
            "profiles__last_trade",
        ),
    };
    IndexedMap::new("profiles", indexes)
}
