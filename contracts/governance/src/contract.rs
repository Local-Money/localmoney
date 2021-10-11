use crate::errors::GovernanceError;
use crate::querier::load_token_balance;
use crate::state::{
    config_read, config_store, stakers_read, stakers_store, state_read, state_store,
};
use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use localterra_protocol::factory_util::get_factory_config;
use localterra_protocol::governance::{
    Config, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, Staker, State,
};

//TODO: Although this contract is named Governance, it doesn't contain any governance function.
// Governance features will be added to this contract on a future upgrade.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, GovernanceError> {
    let config = Config {
        factory_addr: info.sender.clone(),
    };
    let state = State {
        total_shares: Uint128::zero(),
    };
    config_store(deps.storage).save(&config).unwrap();
    state_store(deps.storage).save(&state).unwrap();

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, GovernanceError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::Withdraw { shares } => withdraw_tokens(deps, env, info, shares),
    }
}

fn withdraw_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    shares: Uint128,
) -> Result<Response, GovernanceError> {
    let cfg = config_read(deps.storage).load().unwrap();
    let mut state = state_read(deps.storage).load().unwrap();
    let staker_shares = get_staker_shares(&deps.as_ref(), info.sender.to_string());

    //Check amount being withdrawn
    if shares > staker_shares {
        return Err(GovernanceError::AmountError {
            amount: shares,
            min_amount: Uint128::zero(),
            max_amount: staker_shares,
        });
    }

    //Load contract token balance
    let contract_balance = get_token_balance(&deps.as_ref(), &cfg, &env);
    let withdraw_token_amount = shares.multiply_ratio(contract_balance, state.total_shares);

    //Reduce amount from Staker and Total Shares and save it.
    let sender = info.sender.to_string();
    let new_staker_shares = staker_shares - shares;
    state.total_shares -= shares;

    if new_staker_shares == Uint128::zero() {
        stakers_store(deps.storage).remove(sender.as_bytes());
    } else {
        stakers_store(deps.storage)
            .save(sender.as_bytes(), &new_staker_shares)
            .unwrap();
    }
    state_store(deps.storage).save(&state).unwrap();

    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());

    //Send Tokens
    send_tokens(
        &factory_cfg.token_addr,
        &info.sender,
        withdraw_token_amount.u128(),
        "withdraw",
    )
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Staker { address } => to_binary(&query_staker(deps, address)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

fn query_state(deps: Deps) -> StdResult<State> {
    let state = state_read(deps.storage).load()?;
    Ok(state)
}

fn get_staker_shares(deps: &Deps, address: String) -> Uint128 {
    stakers_read(deps.storage)
        .load(address.as_bytes())
        .unwrap_or(Uint128::zero())
}

fn query_staker(deps: Deps, address: String) -> StdResult<Staker> {
    let staker_shares = get_staker_shares(&deps, address.clone());
    Ok(Staker {
        address,
        shares: staker_shares,
    })
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, GovernanceError> {
    //Only our own token can call this contract
    let cfg: Config = config_read(deps.storage).load().unwrap();
    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());

    if factory_cfg.token_addr != info.sender {
        return Err(GovernanceError::ExecutionError {
            message: "unauthorized".to_string(),
        });
    }

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::StakeTokens {}) => stake_tokens(deps, env, cw20_msg, cfg),
        Ok(Cw20HookMsg::DepositRewards {}) => deposit_rewards(deps.as_ref(), info, cfg),
        Err(_) => Err(GovernanceError::ExecutionError {
            message: "invalid message".to_string(),
        }),
    }
}

fn get_token_balance(deps: &Deps, cfg: &Config, env: &Env) -> Uint128 {
    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());
    load_token_balance(
        &deps.querier,
        factory_cfg.token_addr.to_string(),
        &env.contract.address,
    )
    .unwrap()
}

pub fn stake_tokens(
    deps: DepsMut,
    env: Env,
    cw20_msg: Cw20ReceiveMsg,
    cfg: Config,
) -> Result<Response, GovernanceError> {
    let amount = cw20_msg.amount;
    let sender = cw20_msg.sender;

    if amount.is_zero() {
        return Err(GovernanceError::ExecutionError {
            message: "Insufficient funds sent.".to_string(),
        });
    }

    //Reduce sent amount from balance (it's already increased at this point)
    let contract_balance = get_token_balance(&deps.as_ref(), &cfg, &env);

    let mut state = state_read(deps.storage).load().unwrap();
    let shares = if contract_balance.is_zero() || state.total_shares.is_zero() {
        amount
    } else {
        amount.multiply_ratio(state.total_shares, contract_balance)
    };

    let mut staker_shares = stakers_read(deps.storage)
        .load(sender.as_bytes())
        .unwrap_or(Uint128::zero());

    state.total_shares += shares;
    staker_shares += shares;

    stakers_store(deps.storage)
        .save(sender.as_bytes(), &staker_shares)
        .unwrap();
    state_store(deps.storage).save(&state).unwrap();

    Ok(Response::default())
}

pub fn deposit_rewards(
    deps: Deps,
    info: MessageInfo,
    cfg: Config,
) -> Result<Response, GovernanceError> {
    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());
    if info.sender.eq(&factory_cfg.token_addr) {
        Ok(Response::default())
    } else {
        Err(GovernanceError::ExecutionError {
            message: "Invalid asset.".to_string(),
        })
    }
}

fn send_tokens(
    asset_token: &Addr,
    recipient: &Addr,
    amount: u128,
    action: &str,
) -> Result<Response, GovernanceError> {
    let attributes = vec![
        attr("action", action),
        attr("recipient", recipient.to_string()),
        attr("amount", &amount.to_string()),
    ];

    let mut r = Response::new();
    let cw20msg = &Cw20ExecuteMsg::Transfer {
        recipient: recipient.to_string(),
        amount: Uint128::from(amount),
    };
    r.attributes = attributes;
    r.messages = vec![SubMsg::new(WasmMsg::Execute {
        contract_addr: asset_token.to_string(),
        msg: to_binary(cw20msg).unwrap(),
        funds: vec![],
    })];
    Ok(r)
}
