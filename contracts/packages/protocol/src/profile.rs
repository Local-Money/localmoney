use crate::{trade::TradeState};
use cosmwasm_std::{
    to_binary, Addr, Deps, Order, QuerierWrapper, QueryRequest, StdResult, Storage, WasmQuery,
};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
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
    Profiles { limit: u32, skip: Option<u32> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

// Query Util
pub fn load_profile(
    querier: &QuerierWrapper,
    profile_address: Addr,
    profile_contract: String,
) -> Profile {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: profile_contract,
            msg: to_binary(&QueryMsg::Profile {
                address: profile_address.clone(),
            })
            .unwrap(),
        }))
        .unwrap_or(Profile::new(profile_address))
}

// Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Profile {
    pub address: Addr,
    pub trades_count: u64,
}

impl Profile {
    pub const fn new(address: Addr) -> Self {
        Profile {
            address,
            trades_count: 0,
        }
    }
}

pub struct ProfileModel<'a> {
    pub profile: Profile,
    pub storage: &'a mut dyn Storage,
}

impl ProfileModel<'_> {
    pub fn store(storage: &mut dyn Storage, profile: &Profile) -> StdResult<()> {
        profiles().save(storage, profile.address.to_string(), &profile)
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

    pub fn query(deps: Deps, limit: u32, skip: Option<u32>) -> StdResult<Vec<Profile>> {
        let skip: usize = match skip {
            Some(s) => s as usize,
            None => 0,
        };
        let range = profiles()
            .idx
            .trades_count
            .range(deps.storage, None, None, Order::Descending)
            .skip(skip);
        let result = range
            .take(limit as usize)
            .flat_map(|item| item.and_then(|(_, profile)| Ok(profile)))
            .collect();

        Ok(result)
    }
}

pub struct ProfileIndexes<'a> {
    pub address: MultiIndex<'a, String, Profile, String>,
    pub trades_count: MultiIndex<'a, u64, Profile, String>,
}

impl<'a> IndexList<Profile> for ProfileIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Profile>> + '_> {
        let v: Vec<&dyn Index<Profile>> = vec![&self.address, &self.trades_count];
        Box::new(v.into_iter())
    }
}

pub fn profiles<'a>() -> IndexedMap<'a, String, Profile, ProfileIndexes<'a>> {
    let indexes = ProfileIndexes {
        address: MultiIndex::new(
            |p: &Profile| p.address.to_string(),
            "profiles",
            "profiles__address",
        ),
        trades_count: MultiIndex::new(
            |p: &Profile| p.trades_count,
            "profiles",
            "profiles__trades_count",
        ),
    };
    IndexedMap::new("profiles", indexes)
}
