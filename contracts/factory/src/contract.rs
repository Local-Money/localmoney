#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128, WasmMsg,
};

use crate::error::FactoryError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cw20::Cw20Coin;
use terraswap::token::InstantiateMsg as Cw20InitMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, FactoryError> {
    let response = Response {
        messages: vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: msg.cw20_code_id,
            msg: to_binary(&Cw20InitMsg {
                name: "LocalTerra's Token".to_string(),
                symbol: "LOCAL".to_string(),
                decimals: 6u8,
                initial_balances: vec![Cw20Coin {
                    //TODO: Read from config, don't commit any address.
                    address: env.contract.address.to_string(),
                    amount: Uint128::from(400_000_000 * 1_000_000u128),
                }],
                mint: None,
            })?,
            funds: vec![],
            label: "create token".to_string(),
        }))],
        attributes: vec![],
        events: vec![],
        data: None,
    };
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, FactoryError> {
    Ok(Response::default())
}
