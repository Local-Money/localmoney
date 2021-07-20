#![cfg(test)]
use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Storage, SubMsg, Uint128, WasmMsg,
};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw20::Cw20ExecuteMsg;
use serde::de::StdError;
use serde::{Deserialize, Serialize};
use terraswap::asset::{Asset, AssetInfo, AssetInfo::Token as AssetInfoToken};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub pair: [AssetInfo; 2],
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Swap {
        offer_asset: Asset,
        belief_price: Option<Decimal>,
        max_spread: Option<Decimal>,
        to: Option<String>,
    },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Pair {},
    Pool {},
    Simulation { offer_asset: Asset },
    ReverseSimulation { ask_asset: Asset },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SimulationResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] Box<dyn StdError>),
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    config(deps.storage).save(&msg).unwrap();
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Swap {
            offer_asset,
            belief_price: _,
            max_spread: _,
            to,
        } => {
            if offer_asset.is_native_token() {
                let state = config(deps.storage).load().unwrap();
                let asset_info = state.pair.iter().find(|p| !p.is_native_token()).unwrap();
                let mut token_contract: Option<Addr> = None;
                match asset_info {
                    AssetInfoToken { contract_addr } => {
                        token_contract = Some(contract_addr.clone());
                    }
                    AssetInfo::NativeToken { .. } => {}
                }

                let sub_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: token_contract.unwrap().to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: to.unwrap(),
                        amount: offer_asset.amount.clone(),
                    })
                    .unwrap(),
                    funds: vec![],
                });
                Ok(Response {
                    messages: vec![SubMsg::new(sub_msg)],
                    attributes: vec![],
                    events: vec![],
                    data: None,
                })
            } else {
                let coin = Coin {
                    denom: "uusd".to_string(),
                    amount: offer_asset.amount,
                };
                Ok(Response {
                    messages: vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                        to_address: to.unwrap_or(info.sender.to_string()),
                        amount: vec![coin],
                    }))],
                    attributes: vec![],
                    events: vec![],
                    data: None,
                })
            }
        }
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pair { .. } => Ok(to_binary("").unwrap()),
        QueryMsg::Pool { .. } => Ok(to_binary("").unwrap()),
        QueryMsg::Simulation { offer_asset } => Ok(to_binary(&SimulationResponse {
            return_amount: offer_asset.amount.clone(),
            spread_amount: Uint128::zero(),
            commission_amount: Uint128::zero(),
        })
        .unwrap()),
        QueryMsg::ReverseSimulation { .. } => Ok(to_binary("").unwrap()),
    }
}

pub static CONFIG_KEY: &[u8] = b"config";

pub fn config(storage: &mut dyn Storage) -> Singleton<InstantiateMsg> {
    singleton(storage, CONFIG_KEY)
}

pub fn _config_read(storage: &dyn Storage) -> ReadonlySingleton<InstantiateMsg> {
    singleton_read(storage, CONFIG_KEY)
}
