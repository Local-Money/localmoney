#![cfg(test)]
use cosmwasm_std::{entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, StdError};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use serde::{Deserialize, Serialize};
use terraswap::asset::{AssetInfo, AssetInfo::Token as AssetInfoToken, PairInfo};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub pair_address: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteMsg {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Pair {
        asset_infos: [AssetInfo; 2],
    },
    Pairs {
        start_after: Option<[AssetInfo; 2]>,
        limit: Option<u32>,
    },
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    config(deps.storage)
        .save(&State {
            pair_address: msg.pair_address,
        })
        .unwrap();
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    match msg {
        QueryMsg::Config { .. } => Ok(to_binary("").unwrap()),
        QueryMsg::Pair { .. } => Ok(to_binary(&PairInfo {
            asset_infos: [
                AssetInfoToken {
                    contract_addr: "token".to_string(),
                },
                AssetInfo::NativeToken {
                    denom: "uusd".to_string(),
                },
            ],
            contract_addr: state.pair_address.to_string(),
            liquidity_token: "lptoken".to_string(),
        })
        .unwrap()),
        QueryMsg::Pairs { .. } => Ok(to_binary("").unwrap()),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pair_address: Addr,
}

pub static CONFIG_KEY: &[u8] = b"config";

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
