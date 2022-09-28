use crate::trade::TradeState;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, QuerierWrapper, QueryRequest, Storage, SubMsg, WasmMsg, WasmQuery,
};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const PROFILE_KEY: &str = "localmoney.io:profile";
const PROFILE: Map<String, Profile> = Map::new(PROFILE_KEY);

// Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create {
        profile_address: Addr,
    },
    UpdateProfile {
        profile_address: Addr,
        contact: String,
        encrypt_pk: String,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

// Execute Util
pub fn update_profile_msg(
    profile_contract_addr: String,
    profile_address: Addr,
    contact: String,
    encrypt_pk: String,
) -> SubMsg {
    SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_contract_addr,
        msg: to_binary(&ExecuteMsg::UpdateProfile {
            profile_address,
            contact,
            encrypt_pk,
        })
        .unwrap(),
        funds: vec![],
    }))
}

pub fn increase_profile_trades_count_msg(
    profile_contract_addr: String,
    profile_address: Addr,
    final_trade_state: TradeState,
) -> SubMsg {
    SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: profile_contract_addr,
        msg: to_binary(&ExecuteMsg::IncreaseTradeCount {
            profile_address,
            final_trade_state,
        })
        .unwrap(),
        funds: vec![],
    }))
}

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
    pub trade_count: u64,
    pub contact: Option<String>,
    pub encrypt_pk: Option<String>,
}

impl Profile {
    pub const fn new(address: Addr) -> Self {
        Profile {
            address,
            trade_count: 0,
            contact: None,
            encrypt_pk: None,
        }
    }
}

// Model
pub struct ProfileModel<'a> {
    pub profile: Profile,
    pub storage: &'a mut dyn Storage,
}

impl ProfileModel<'_> {
    pub fn has(storage: &mut dyn Storage, profile_address: String) -> bool {
        PROFILE.has(storage, profile_address)
    }

    pub fn query(storage: &dyn Storage, profile_address: Addr) -> Profile {
        PROFILE
            .may_load(storage, profile_address.to_string())
            .unwrap()
            .unwrap_or(Profile::new(profile_address.clone()))
    }

    pub fn load<'a>(storage: &'a mut dyn Storage, profile_address: Addr) -> ProfileModel<'a> {
        let profile = ProfileModel::query(storage, profile_address);
        ProfileModel { profile, storage }
    }

    pub fn store<'a>(storage: &'a mut dyn Storage, profile: &Profile) -> ProfileModel<'a> {
        PROFILE
            .save(storage, profile.address.to_string(), &profile)
            .unwrap();

        ProfileModel {
            profile: profile.clone(),
            storage,
        }
    }

    pub fn save(&mut self) -> Profile {
        ProfileModel::store(self.storage, &self.profile).profile
    }
}
