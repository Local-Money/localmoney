use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Order, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};

use localterra_protocol::constants::VAULT_TIMEOUT;

use crate::error::ContractError;
use crate::state::{claims, Claim, Config, State, CONFIG, STATE};
use astroport::staking::{ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use cw2::set_contract_version;
use cw20::{
    BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg, MinterResponse,
    TokenInfoResponse,
};

use crate::response::MsgInstantiateContractResponse;
use astroport::asset::addr_validate_to_lower;
use astroport::token::InstantiateMsg as TokenInstantiateMsg;
use protobuf::Message;

/// Contract name that is used for migration.
const CONTRACT_NAME: &str = "localterra-staking";
/// Contract version that is used for migration.
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// xLOCAL information.
const TOKEN_NAME: &str = "Staked LOCAL";
const TOKEN_SYMBOL: &str = "xLOCAL";

/// A `reply` call code ID used for sub-messages.
const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

/// ## Description
/// Creates a new contract with the specified parameters in the [`InstantiateMsg`].
/// Returns a [`Response`] with the specified attributes if the operation was successful,
/// or a [`ContractError`] if the contract was not created.
/// ## Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **env** is an object of type [`Env`].
///
/// * **_info** is an object of type [`MessageInfo`].
///
/// * **msg** is a message of type [`InstantiateMsg`] which contains the parameters for creating the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Store config
    CONFIG.save(
        deps.storage,
        &Config {
            local_token_addr: addr_validate_to_lower(deps.api, &msg.deposit_token_addr)?,
            xlocal_token_addr: Addr::unchecked(""),
        },
    )?;

    // Store claims count
    STATE.save(
        deps.storage,
        &State {
            claims_count: 0,
            total_local_warming: Uint128::from(0u32),
        },
    )?;

    // Create the xLOCAL token
    let sub_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            admin: Some(msg.owner),
            code_id: msg.token_code_id,
            msg: to_binary(&TokenInstantiateMsg {
                name: TOKEN_NAME.to_string(),
                symbol: TOKEN_SYMBOL.to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: env.contract.address.to_string(),
                    cap: None,
                }),
            })?,
            funds: vec![],
            label: String::from("Staked Local Token"),
        }
        .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new().add_submessages(sub_msg))
}

/// ## Description
/// Exposes execute functions available in the contract.
/// ## Params
/// * **deps** is an object of type [`Deps`].
///
/// * **env** is an object of type [`Env`].
///
/// * **_info** is an object of type [`MessageInfo`].
///
/// * **msg** is an object of type [`ExecuteMsg`].
///
/// ## Queries
/// * **ExecuteMsg::Receive(msg)** Receives a message of type [`Cw20ReceiveMsg`] and processes
/// it depending on the received template.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::Claim { claim_id } => execute_claim(deps, env, info, claim_id),
    }
}

/// ## Description
/// The entry point to the contract for processing replies from submessages. For now it only sets the xLOCAL contract address.
/// # Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **_env** is an object of type [`Env`].
///
/// * **msg** is an object of type [`Reply`].
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.xlocal_token_addr != Addr::unchecked("") {
        return Err(ContractError::Unauthorized {});
    }

    let data = msg.result.unwrap().data.unwrap();
    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(data.as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;

    // Set xLOCAL addr
    config.xlocal_token_addr = addr_validate_to_lower(deps.api, res.get_contract_address())?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

/// ## Description
/// Receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received template.
/// If the template is not found in the received message, then a [`ContractError`] is returned,
/// otherwise returns a [`Response`] with the specified attributes if the operation was successful
/// ## Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **env** is an object of type [`Env`].
///
/// * **info** is an object of type [`MessageInfo`].
///
/// * **cw20_msg** is an object of type [`Cw20ReceiveMsg`]. This is the CW20 message to process.
fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    let recipient = cw20_msg.sender;
    let amount = cw20_msg.amount;

    let mut total_deposit = get_total_deposit(deps.as_ref(), env.clone(), config.clone())?;
    // We need to subtract the total local warming (locals waiting for maturity to be claimed)
    // so we don't reissue shares for them a second time
    let mut state: State = STATE.load(deps.storage)?;
    total_deposit -= state.total_local_warming; // TODO test this / try to break it
    let total_shares = get_total_shares(deps.as_ref(), config.clone())?;

    match from_binary(&cw20_msg.msg)? {
        Cw20HookMsg::Enter {} => {
            if info.sender != config.local_token_addr {
                return Err(ContractError::Unauthorized {});
            }
            // In a CW20 `send`, the total balance of the recipient is already increased.
            // To properly calculate the total amount of LOCAL deposited in staking, we should subtract the user deposit from the pool
            total_deposit -= amount;

            let mint_amount: Uint128 = if total_shares.is_zero() || total_deposit.is_zero() {
                amount
            } else {
                amount
                    .checked_mul(total_shares)?
                    .checked_div(total_deposit)
                    .map_err(|e| StdError::DivideByZero { source: e })?
            };

            let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.xlocal_token_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Mint {
                    recipient,
                    amount: mint_amount,
                })?,
                funds: vec![],
            }));

            Ok(res)
        }
        Cw20HookMsg::Leave {} => {
            if info.sender != config.xlocal_token_addr {
                return Err(ContractError::Unauthorized {});
            }

            // The LOCAL amount to transfer after reaching maturity
            let what = amount
                .checked_mul(total_deposit)?
                .checked_div(total_shares)
                .map_err(|e| StdError::DivideByZero { source: e })?;

            // Burn share (xLOCAL)
            let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.xlocal_token_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Burn { amount })?,
                funds: vec![],
            }));
            // .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            //     contract_addr: config.local_token_addr.to_string(),
            //     msg: to_binary(&Cw20ExecuteMsg::Transfer {
            //         recipient,
            //         amount: what,
            //     })?,
            //     funds: vec![],
            // }));

            // Update state with claim_id and total_local_warming

            let claim_id = state.claims_count + 1;

            state.claims_count = claim_id;

            state.total_local_warming += what;

            STATE.save(deps.storage, &state)?;

            // Save this claim so it can be queried and transferred upon reaching maturity.
            claims().save(
                deps.storage,
                &claim_id.to_string(),
                &Claim {
                    id: claim_id,
                    recipient: deps.api.addr_validate(&recipient)?,
                    amount: what,
                    created_at: env.block.time.seconds(),
                },
            )?;

            Ok(res)
        }
    }
}

fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    claim_id: u64,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    let claim = claims()
        .may_load(deps.storage, &claim_id.to_string())
        .unwrap_or_default()
        .unwrap(); // TODO handle error if claim doesn't exist

    if info.sender != claim.recipient {
        return Err(ContractError::Unauthorized {});
    }

    if env.block.time.seconds() < claim.created_at + VAULT_TIMEOUT {
        return Err(ContractError::Immature {});
    }

    // Remove claim so it can't be replayed
    claims()
        .remove(deps.storage, &claim_id.to_string())
        .unwrap();

    // Reduce the total local warming amount so total deposits are calculated correctly
    state.total_local_warming -= claim.amount;
    STATE.save(deps.storage, &state)?;

    // Transfer matured LOCAL
    let res = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.local_token_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: claim.recipient.to_string(),
            amount: claim.amount,
        })?,
        funds: vec![],
    }));

    Ok(res)
}

pub fn query_claims(deps: Deps, recipient: Addr) -> StdResult<Vec<Claim>> {
    let storage = deps.storage;

    let result = claims()
        .idx
        .recipient
        .prefix(recipient)
        .range(storage, None, None, Order::Descending)
        // .take(10) // set defaults
        .flat_map(|item| item.and_then(|(_, recipient)| Ok(recipient)))
        .collect();

    Ok(result)
}

/// ## Description
/// Returns the total amount of xLOCAL currently issued.
/// ## Params
/// * **deps** is an object of type [`Deps`].
///
/// * **config** is an object of type [`Config`]. This is the staking contract configuration.
pub fn get_total_shares(deps: Deps, config: Config) -> StdResult<Uint128> {
    let result: TokenInfoResponse = deps
        .querier
        .query_wasm_smart(&config.xlocal_token_addr, &Cw20QueryMsg::TokenInfo {})?;

    Ok(result.total_supply)
}

/// ## Description
/// Returns the total amount of LOCAL deposited in the contract.
/// ## Params
/// * **deps** is an object of type [`Deps`].
///
/// * **env** is an object of type [`Env`].
///
/// * **config** is an object of type [`Config`]. This is the staking contract configuration.
pub fn get_total_deposit(deps: Deps, env: Env, config: Config) -> StdResult<Uint128> {
    let result: BalanceResponse = deps.querier.query_wasm_smart(
        &config.local_token_addr,
        &Cw20QueryMsg::Balance {
            address: env.contract.address.to_string(),
        },
    )?;
    Ok(result.balance)
}

/// ## Description
/// Returns the total amount of LOCAL warming until maturity is reach and they can be claimed.
/// ## Params
/// * **deps** is an object of type [`Deps`].
pub fn get_total_local_warming(deps: Deps) -> StdResult<Uint128> {
    let state: State = STATE.load(deps.storage)?;

    Ok(state.total_local_warming)
}

/// ## Description
/// Exposes all the queries available in the contract.
/// # Params
/// * **deps** is an object of type [`DepsMut`].
///
/// * **env** is an object of type [`Env`].
///
/// * **msg** is an object of type [`QueryMsg`].
///
/// ## Queries
/// * **QueryMsg::Config {}** Returns the staking contract configuration using a [`ConfigResponse`] object.
///
/// * **QueryMsg::TotalShares {}** Returns the total xLOCAL supply using a [`Uint128`] object.
///
/// * **QueryMsg::Config {}** Returns the amount of LOCAL that's currently in the staking pool using a [`Uint128`] object.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    match msg {
        QueryMsg::Config {} => Ok(to_binary(&ConfigResponse {
            deposit_token_addr: config.local_token_addr,
            share_token_addr: config.xlocal_token_addr,
        })?),
        QueryMsg::TotalShares {} => to_binary(&get_total_shares(deps, config)?),
        QueryMsg::TotalDeposit {} => to_binary(&get_total_deposit(deps, env, config)?),
        QueryMsg::TotalWarming {} => to_binary(&get_total_local_warming(deps)?),
        QueryMsg::Claims { recipient } => to_binary(&query_claims(deps, recipient)?),
    }
}
