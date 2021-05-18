use cosmwasm_std::{
    to_binary, Api, BankMsg, Binary, Coin, CosmosMsg, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, LogAttribute, Querier, QueryRequest, StdError, StdResult, Storage, Uint128,
    WasmQuery,
};

use crate::msg::{ConfigResponse, HandleMsg, InitMsg, OfferMsg, QueryMsg};
use crate::state::{config, config_read, OfferResponse, OfferType, State, TradeState};
use crate::taxation::deduct_tax;

use std::cmp::min;
use std::ops::Sub;
use terra_cosmwasm::{TaxCapResponse, TaxRateResponse, TerraQuerier};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
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
        sender = env.message.sender.clone();
    } else {
        recipient = env.message.sender.clone();
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

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    let mut cfg = config(&mut deps.storage);
    let mut state = cfg.load()?;
    if !env.message.sent_funds.is_empty() {
        let balance = deps.querier.query_balance(&env.contract.address, "uusd")?;
        if balance.amount >= state.amount {
            state.state = TradeState::EscrowFunded;
        }
        cfg.save(&state)?;
    }
    // let mut cfg = config(&mut deps.storage);
    // let state = cfg.load()?;
    match msg {
        HandleMsg::Refund {} => try_refund(deps, env, msg, state),
        HandleMsg::Release {} => try_release(deps, env, msg, state),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ConfigResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(state)
}

fn try_release<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: HandleMsg,
    state: State,
) -> StdResult<HandleResponse> {
    if env.message.sender != state.sender {
        return Err(StdError::unauthorized());
    }

    // throws error if state is expired
    if env.block.height > state.expire_height {
        return Err(StdError::generic_err("This trade has expired"));
    }

    let mut balance = deps.querier.query_all_balances(&env.contract.address)?;

    balance[0].amount = deduct_tax(&deps, balance[0].clone()).unwrap().amount;

    let mut cfg = config(&mut deps.storage);
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

fn try_refund<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: HandleMsg,
    state: State,
) -> StdResult<HandleResponse> {
    // anyone can try to refund, as long as the contract is expired
    if env.block.height > state.expire_height {
        return Err(StdError::generic_err("Can't release an unexpired Trade."));
    }

    let mut balance = deps.querier.query_all_balances(&env.contract.address)?;
    let querier = TerraQuerier::new(&deps.querier);
    let tax_cap: TaxCapResponse = querier.query_tax_cap("uusd")?;
    let tax_rate: TaxRateResponse = querier.query_tax_rate()?;
    let tax = min(balance[0].amount * tax_rate.rate, tax_cap.cap);
    balance[0].amount = balance[0].amount.sub(tax)?;

    send_tokens(deps, env.contract.address, state.sender, balance, "refund")
}

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    from_address: HumanAddr,
    to_address: HumanAddr,
    amount: Vec<Coin>,
    action: &str,
) -> StdResult<HandleResponse> {
    let attributes = vec![attr("action", action), attr("to", to_address.clone())];
    let amount = [deduct_tax(deps, amount[0].clone()).unwrap()].to_vec();
    println!(
        "Sending {}{} from {} to {}",
        amount[0].amount,
        amount[0].denom,
        from_address.clone(),
        to_address.clone()
    );
    let r = HandleResponse {
        messages: vec![CosmosMsg::Bank(BankMsg::Send {
            from_address,
            to_address,
            amount,
        })],
        data: None,
        log: attributes,
    };
    Ok(r)
}

pub fn attr<K: ToString, V: ToString>(key: K, value: V) -> LogAttribute {
    LogAttribute {
        key: key.to_string(),
        value: value.to_string(),
    }
}
