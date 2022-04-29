use cosmwasm_std::{
    entry_point, Addr, Binary, ContractResult, Deps, Reply, ReplyOn, StdError, StdResult,
    SubMsgExecutionResponse,
};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};

use crate::errors::FactoryError;
use crate::state::CONFIG;
use astroport::staking::InstantiateMsg as StakingInstantiateMsg;
use localterra_protocol::factory::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use localterra_protocol::governance::InstantiateMsg as GovernanceInstantiateMsg;
use localterra_protocol::offer::InstantiateMsg as OfferInstantiate;
use localterra_protocol::trading_incentives::InstantiateMsg as TradingIncentivesInstantiateMsg;
// use terraswap::token::InstantiateMsg as TokenInstantiateMsg;

pub const GOV_REPLY_ID: u64 = 0;
pub const OFFER_REPLY_ID: u64 = 2;
pub const TRADING_INCENTIVES_REPLY_ID: u64 = 3;
pub const CW20_TOKEN_REPLY_ID: u64 = 4;
pub const STAKING_REPLY_ID: u64 = 4;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, FactoryError> {
    let cfg = Config {
        trade_code_id: msg.trade_code_id,
        token_addr: Addr::unchecked(""), // TODO refactor to local_token_addr
        local_ust_pool_addr: deps.api.addr_validate(&msg.local_ust_pool_addr).unwrap(),
        gov_addr: Addr::unchecked(""),
        offers_addr: Addr::unchecked(""),
        trading_incentives_addr: Addr::unchecked(""),
        staking_addr: Addr::unchecked(""),
    };
    CONFIG.save(deps.storage, &cfg).unwrap();

    let offer_msg = instantiate_offer_msg(msg.offer_code_id);
    let gov_msg = instantiate_gov_msg(msg.gov_contract_code_id);
    let trading_incentives_msg = instantiate_trading_incentives_msg(msg.trading_incentives_code_id);
    let staking_msg =
        instantiate_staking_msg(msg.local_token_addr, msg.staking_code_id, msg.cw20_code_id);

    let r = Response::new()
        .add_submessage(offer_msg)
        .add_submessage(trading_incentives_msg)
        .add_submessage(gov_msg)
        .add_submessage(staking_msg);
    Ok(r)
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

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, FactoryError> {
    match msg.id {
        GOV_REPLY_ID => instantiate_gov_reply(deps, msg.result),
        OFFER_REPLY_ID => instantiate_offer_reply(deps, msg.result),
        TRADING_INCENTIVES_REPLY_ID => instantiate_trading_incentives_reply(deps, msg.result),
        STAKING_REPLY_ID => instantiate_staking_reply(deps, msg.result),
        _ => Err(FactoryError::Std(StdError::generic_err(
            "Unknown reply id.",
        ))),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage).unwrap()),
    }
}

fn instantiate_gov_msg(code_id: u64) -> SubMsg {
    create_instantiate_msg(
        code_id,
        to_binary(&GovernanceInstantiateMsg {}).unwrap(),
        GOV_REPLY_ID,
        "gov".to_string(),
    )
}

fn instantiate_gov_reply(
    deps: DepsMut,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, FactoryError> {
    if result.is_err() {
        return Err(FactoryError::Std(StdError::generic_err(
            "Failed to instantiate gov contract.",
        )));
    }

    let gov_contract_addr = get_contract_address_from_reply(deps.as_ref(), result);
    let mut cfg = CONFIG.load(deps.storage).unwrap();
    cfg.gov_addr = gov_contract_addr.clone();
    CONFIG.save(deps.storage, &cfg).unwrap();
    let res = Response::new().add_attribute("instantiate_contract", "gov");
    Ok(res)
}

fn instantiate_offer_msg(code_id: u64) -> SubMsg {
    create_instantiate_msg(
        code_id,
        to_binary(&OfferInstantiate {}).unwrap(),
        OFFER_REPLY_ID,
        "offer".to_string(),
    )
}

fn instantiate_offer_reply(
    deps: DepsMut,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, FactoryError> {
    if result.is_err() {
        return Err(FactoryError::Std(StdError::generic_err(
            "Failed to instantiate offer contract.",
        )));
    }

    let offer_addr = get_contract_address_from_reply(deps.as_ref(), result);
    let mut cfg = CONFIG.load(deps.storage).unwrap();
    cfg.offers_addr = offer_addr;
    CONFIG.save(deps.storage, &cfg).unwrap();
    let res = Response::new().add_attribute("instantiate_contract", "offers");
    Ok(res)
}

fn instantiate_staking_msg(
    local_token_addr: String,
    staking_code_id: u64,
    cw20_code_id: u64,
) -> SubMsg {
    create_instantiate_msg(
        staking_code_id,
        to_binary(&StakingInstantiateMsg {
            owner: "terra1vpy55n57punft83gsa9gte7rhruzm9sehav2wp".to_string(), // TODO make dynamic
            token_code_id: cw20_code_id,
            deposit_token_addr: local_token_addr,
        })
        .unwrap(),
        STAKING_REPLY_ID,
        "instantiate-staking".to_string(),
    )
}
fn instantiate_staking_reply(
    deps: DepsMut,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, FactoryError> {
    if result.is_err() {
        return Err(FactoryError::Std(StdError::generic_err(
            "Failed to instantiate token contract.",
        )));
    }

    let mut cfg = CONFIG.load(deps.storage).unwrap();
    cfg.staking_addr = get_contract_address_from_reply(deps.as_ref(), result);
    CONFIG.save(deps.storage, &cfg).unwrap();
    let res = Response::new().add_attribute("instantiate_contract", "staking");
    Ok(res)
}

// fn instantiate_token_msg(cw20_code_id: u64) -> SubMsg {
//     create_instantiate_msg(
//         cw20_code_id,
//         to_binary(&TokenInstantiateMsg {
//             name: "LocalTerra's Token".to_string(),
//             symbol: "LOCAL".to_string(),
//             decimals: 6u8,
//             initial_balances: vec![],
//             mint: None,
//         })
//         .unwrap(),
//         CW20_TOKEN_REPLY_ID,
//         "create-token".to_string(),
//     )
// }

// fn instantiate_token_reply(
//     deps: DepsMut,
//     result: ContractResult<SubMsgExecutionResponse>,
// ) -> Result<Response, FactoryError> {
//     if result.is_err() {
//         return Err(FactoryError::Std(StdError::generic_err(
//             "Failed to instantiate token contract.",
//         )));
//     }

//     let mut cfg = CONFIG.load(deps.storage).unwrap();
//     cfg.token_addr = get_contract_address_from_reply(deps.as_ref(), result);
//     CONFIG.save(deps.storage, &cfg).unwrap();
//     let res = Response::new().add_attribute("instantiate_contract", "token");
//     Ok(res)
// }

fn instantiate_trading_incentives_msg(trading_incentives_code_id: u64) -> SubMsg {
    create_instantiate_msg(
        trading_incentives_code_id,
        to_binary(&TradingIncentivesInstantiateMsg {}).unwrap(),
        TRADING_INCENTIVES_REPLY_ID,
        "trading-incentives".to_string(),
    )
}

fn instantiate_trading_incentives_reply(
    deps: DepsMut,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, FactoryError> {
    if result.is_err() {
        return Err(FactoryError::Std(StdError::generic_err(
            "Failed to instantiate trading incentives contract.",
        )));
    }

    let mut cfg = CONFIG.load(deps.storage).unwrap();
    cfg.trading_incentives_addr = get_contract_address_from_reply(deps.as_ref(), result);
    CONFIG.save(deps.storage, &cfg).unwrap();
    let res = Response::new().add_attribute("instantiate_contract", "incentives");
    Ok(res)
}

fn get_contract_address_from_reply(
    deps: Deps,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Addr {
    result
        .unwrap()
        .events
        .into_iter()
        .find(|e| e.ty == "instantiate_contract")
        .and_then(|ev| {
            ev.attributes
                .into_iter()
                .find(|attr| attr.key == "contract_address")
        })
        .map(|attr| deps.api.addr_validate(attr.value.as_str()).unwrap())
        .unwrap()
}

fn create_instantiate_msg(code_id: u64, msg: Binary, reply_id: u64, label: String) -> SubMsg {
    let instantiate_msg = WasmMsg::Instantiate {
        admin: None,
        code_id,
        msg,
        funds: vec![],
        label,
    };
    SubMsg {
        id: reply_id,
        msg: CosmosMsg::Wasm(instantiate_msg),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }
}
