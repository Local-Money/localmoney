use cosmwasm_std::{to_binary, Attribute, BankMsg, Binary, Coin, CosmosMsg, Env, HandleResponse, HumanAddr, InitResponse, MessageInfo, StdError, StdResult, Uint128, WasmQuery, DepsMut, Deps, QueryRequest};

use crate::msg::{ConfigResponse, HandleMsg, InitMsg, OfferMsg, QueryMsg};
use crate::state::{config, config_read, OfferResponse, OfferType, State, TradeState};
use crate::errors::TradeError;

pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse, TradeError> {
    let offer_id = msg.offer;
    let offer: OfferResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: msg.offer_contract,
        msg: to_binary(&OfferMsg::LoadOffer { id: offer_id })?,
    }))?;

    //TODO: it's probably a good idea to store this kind of configuration in a Gov contract.
    let expire_height = env.block.height + 100; //Roughly 10 Minutes.
    let recipient: HumanAddr;
    let sender: HumanAddr;

    if offer.offer_type == OfferType::Buy {
        recipient = offer.owner;
        sender = info.sender.clone();
    } else {
        recipient = info.sender.clone();
        sender = offer.owner;
    }

    let mut state = State {
        recipient,
        sender,
        offer_id,
        state: TradeState::Created,
        expire_height,
        amount: Uint128::from(msg.amount),
    };

    let amount_sent = deps.querier.query_balance(&env.contract.address, "uusd")?;
    if amount_sent.amount >= Uint128::from(msg.amount) {
        state.state = TradeState::EscrowFunded
    }

    config(deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, TradeError> {
    let mut cfg = config(deps.storage);
    let mut state = cfg.load()?;
    if !info.sent_funds.is_empty() {
        let balance = deps.querier.query_balance(&env.contract.address, "uusd")?;
        if balance.amount >= state.amount {
            state.state = TradeState::EscrowFunded;
        }
        cfg.save(&state)?;
    }
    // let mut cfg = config(&mut deps.storage);
    // let state = cfg.load()?;
    match msg {
        HandleMsg::Refund {} => try_refund(deps, env, info, msg, state),
        HandleMsg::Release {} => try_release(deps, env, info, msg, state),
    }
}

pub fn query(
    deps: Deps,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

fn try_release(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: HandleMsg,
    state: State,
) -> Result<HandleResponse, TradeError> {
    if info.sender != state.sender {
        return Err(TradeError::Std(StdError::generic_err("Unauthorized")));
    }

    // throws error if state is expired
    if env.block.height > state.expire_height {
        return Err(TradeError::Std(StdError::generic_err("This trade has expired")));
    }

    let mut balance = deps.querier.query_all_balances(&env.contract.address)?;
    //TODO: Deduct Tax
    //balance[0].amount = deduct_tax(&deps, balance[0].clone()).unwrap().amount;

    let mut cfg = config(deps.storage);
    let mut state = cfg.load()?;
    state.state = TradeState::Closed;
    cfg.save(&state)?;

    send_tokens(
        deps,
        env.contract.address,
        state.recipient,
        balance,
        "approve",
    )
}

fn try_refund(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: HandleMsg,
    state: State,
) -> Result<HandleResponse, TradeError> {
    // anyone can try to refund, as long as the contract is expired
    if state.expire_height > env.block.height {
        return Err(TradeError::Std(StdError::generic_err("Can't release an unexpired Trade.")));
    }

    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    send_tokens(deps, env.contract.address, state.sender, balance, "refund")
}

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(
    _deps: DepsMut,
    from_address: HumanAddr,
    to_address: HumanAddr,
    amount: Vec<Coin>,
    action: &str,
) -> Result<HandleResponse, TradeError> {
    let attributes = vec![attr("action", action), attr("to", to_address.clone())];
    //TODO
    //let amount = [deduct_tax(deps, amount[0].clone()).unwrap()].to_vec();

    let r = HandleResponse {
        messages: vec![CosmosMsg::Bank(BankMsg::Send {
            from_address,
            to_address,
            amount,
        })],
        data: None,
        attributes,
    };
    Ok(r)
}

pub fn attr<K: ToString, V: ToString>(key: K, value: V) -> Attribute {
    Attribute {
        key: key.to_string(),
        value: value.to_string(),
    }
}
