use crate::{offer::OfferState, trade::TradeState};
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Deps, Env, Order, QuerierWrapper, StdResult, Storage,
    SubMsg, WasmMsg,
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
    UpdateContact {
        profile_addr: Addr,
        contact: String,
        encryption_key: String,
    },
    UpdateActiveOffers {
        profile_addr: Addr,
        offer_state: OfferState,
    },
    UpdateTradesCount {
        profile_addr: Addr,
        trade_state: TradeState,
    },
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Profile { addr: Addr },
    Profiles { limit: u32, start_at: Option<u64> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

// Execute Util
pub fn update_profile_contact_msg(
    profile_contract: String,
    profile_addr: Addr,
    contact: String,
    encryption_key: String,
) -> SubMsg {
    SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_contract,
        msg: to_binary(&ExecuteMsg::UpdateContact {
            profile_addr,
            contact,
            encryption_key,
        })
        .unwrap(),
        funds: vec![],
    }))
}

pub fn update_profile_trades_count_msg(
    contract_addr: String,
    profile_addr: Addr,
    trade_state: TradeState,
) -> SubMsg {
    SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr,
        msg: to_binary(&ExecuteMsg::UpdateTradesCount {
            profile_addr,
            trade_state,
        })
        .unwrap(),
        funds: vec![],
    }))
}

pub fn update_profile_active_offers_msg(
    contract_addr: String,
    profile_addr: Addr,
    offer_state: OfferState,
) -> SubMsg {
    SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr,
        msg: to_binary(&ExecuteMsg::UpdateActiveOffers {
            profile_addr,
            offer_state,
        })
        .unwrap(),
        funds: vec![],
    }))
}

// Query Util
pub fn load_profile<T: CustomQuery>(
    querier: &QuerierWrapper<T>,
    profile_contract: String,
    profile_addr: Addr,
) -> StdResult<Profile> {
    querier.query_wasm_smart(
        profile_contract,
        &QueryMsg::Profile {
            addr: profile_addr.clone(),
        },
    )
}

pub fn load_profiles<T: CustomQuery>(
    querier: &QuerierWrapper<T>,
    profile_contract: String,
    limit: u32,
    start_at: Option<u64>,
) -> StdResult<Vec<Profile>> {
    querier.query_wasm_smart(profile_contract, &QueryMsg::Profiles { limit, start_at })
}

// Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Profile {
    pub addr: Addr,
    pub created_at: u64,
    pub requested_trades_count: u64,
    pub active_trades_count: u8,
    pub released_trades_count: u64,
    pub last_trade: u64,
    pub contact: Option<String>,
    pub encryption_key: Option<String>,
    pub active_offers_count: u8,
}

impl Profile {
    pub fn new(addr: Addr, created_at: u64) -> Self {
        Profile {
            addr,
            created_at,
            released_trades_count: 0,
            requested_trades_count: 0,
            last_trade: 0,
            contact: None,
            encryption_key: None,
            active_offers_count: 0,
            active_trades_count: 0,
        }
    }
}

// Model
pub struct ProfileModel<'a> {
    pub profile: Profile,
    pub storage: &'a mut dyn Storage,
}

impl ProfileModel<'_> {
    pub fn store<'a>(storage: &'a mut dyn Storage, profile: &Profile) -> ProfileModel<'a> {
        profiles()
            .save(storage, profile.addr.to_string(), &profile)
            .unwrap();
        ProfileModel {
            profile: profile.clone(),
            storage,
        }
    }

    pub fn from_store<'a>(
        storage: &'a mut dyn Storage,
        profile_addr: Addr,
    ) -> StdResult<ProfileModel<'a>> {
        match profiles().load(storage, profile_addr.to_string()) {
            Ok(profile) => Ok(ProfileModel { profile, storage }),
            Err(e) => Err(e),
        }
    }

    pub fn query_profile(storage: &dyn Storage, profile_addr: Addr) -> Profile {
        profiles()
            .load(storage, profile_addr.to_string())
            .unwrap_or(Profile::new(profile_addr, 0))
    }

    pub fn save<'a>(self) -> Profile {
        ProfileModel::store(self.storage, &self.profile).profile
    }

    pub fn query_profiles(
        deps: Deps,
        _env: Env,
        limit: u32,
        _start_at: Option<u64>,
    ) -> StdResult<Vec<Profile>> {
        let result = profiles()
            .idx
            .trades_count
            .range(deps.storage, None, None, Order::Descending)
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

const PROFILES_PK: &str = "profiles_v0_4_0";
pub fn profiles<'a>() -> IndexedMap<'a, String, Profile, ProfileIndexes<'a>> {
    let indexes = ProfileIndexes {
        address: MultiIndex::new(
            |p: &Profile| p.addr.to_string(),
            PROFILES_PK,
            "profiles__address",
        ),
        trades_count: MultiIndex::new(
            |p: &Profile| p.released_trades_count,
            PROFILES_PK,
            "profiles__trades_count",
        ),
        last_trade: MultiIndex::new(
            |p: &Profile| p.last_trade,
            PROFILES_PK,
            "profiles__last_trade",
        ),
    };
    IndexedMap::new(PROFILES_PK, indexes)
}
