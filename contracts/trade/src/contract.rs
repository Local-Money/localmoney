use cosmwasm_std::{
    entry_point, to_binary, Addr, Attribute, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Response, StdResult, Uint128, WasmQuery,
};

use crate::errors::TradeError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, OfferMsg, QueryMsg};
use crate::state::{config, config_read, State, TradeState};
use crate::taxation::deduct_tax;
use cosmwasm_storage::Singleton;
use offer::state::{Offer, OfferType};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, TradeError> {
    let offer_id = msg.offer;
    let load_offer_result: StdResult<Offer> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: msg.offer_contract.to_string(),
            msg: to_binary(&OfferMsg::LoadOffer { id: offer_id }).unwrap(),
        }));
    if load_offer_result.is_err() {
        return Err(TradeError::OfferNotFound { offer_id });
    }
    let offer = load_offer_result.unwrap();

    //TODO: it's probably a good idea to store this kind of configuration in a Gov contract.
    let expire_height = env.block.height + 100; //Roughly 10 Minutes.
    let recipient: Addr;
    let sender: Addr;

    let amount = Uint128::from(msg.amount);
    if amount > offer.max_amount || amount < offer.min_amount {
        return Err(TradeError::AmountError {
            amount,
            min_amount: offer.min_amount,
            max_amount: offer.max_amount,
        });
    }

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

    if !info.funds.is_empty() {
        let ust_amount = get_ust_amount(info.clone());
        if ust_amount >= Uint128::from(msg.amount) {
            state.state = TradeState::EscrowFunded
        }
    }

    let save_state_result = config(deps.storage).save(&state);
    if save_state_result.is_err() {
        return Err(TradeError::InstantiationError {
            message: "Couldn't save state.".to_string(),
        });
    }

    Ok(Response::default())
}

fn get_ust_amount(info: MessageInfo) -> Uint128 {
    let mut ust_amount = Uint128::zero();
    let ust_index: &Option<usize> = &info.funds.iter().position(|coin| coin.denom.eq("uusd"));
    if Into::<usize>::into(ust_index.unwrap()) >= usize::MIN {
        let ust_coin: &Coin = &info.funds[ust_index.unwrap()];
        ust_amount = ust_coin.amount;
    }
    return ust_amount;
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, TradeError> {
    let cfg = config(deps.storage);
    let state = cfg.load().unwrap();
    match msg {
        ExecuteMsg::FundEscrow {} => try_fund_escrow(info, cfg, state),
        ExecuteMsg::Refund {} => try_refund(deps, env, state),
        ExecuteMsg::Release {} => try_release(deps, env, info, state),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

fn try_fund_escrow(
    info: MessageInfo,
    mut cfg: Singleton<State>,
    mut state: State,
) -> Result<Response, TradeError> {
    if !info.funds.is_empty() {
        let ust_amount = get_ust_amount(info.clone());
        if ust_amount >= state.amount {
            state.state = TradeState::EscrowFunded;
        }
        let save_result = cfg.save(&state);
        if save_result.is_err() {
            return Err(TradeError::ExecutionError {
                message: "Failed to save state.".to_string(),
            });
        }
    }
    Ok(Response::default())
}

fn try_release(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    state: State,
) -> Result<Response, TradeError> {
    if info.sender != state.sender {
        return Err(TradeError::Unauthorized {
            owner: state.sender,
            caller: info.sender,
        });
    }

    // throws error if state is expired
    if env.block.height > state.expire_height {
        return Err(TradeError::Expired {
            expire_height: state.expire_height,
            current_height: env.block.height,
        });
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    if balance_result.is_err() {
        return Err(TradeError::ReleaseError {
            message: "Contract has no funds.".to_string(),
        });
    }
    let balance = balance_result.unwrap();
    //TODO: Deduct Tax
    //balance[0].amount = deduct_tax(&deps, balance[0].clone()).unwrap().amount;

    let mut cfg = config(deps.storage);
    let mut state = cfg.load().unwrap();
    state.state = TradeState::Closed;
    let save_result = cfg.save(&state);
    if save_result.is_err() {
        return Err(TradeError::ExecutionError {
            message: "Failed to save state.".to_string(),
        });
    }

    send_tokens(deps, state.recipient, balance, "approve")
}

fn try_refund(deps: DepsMut, env: Env, state: State) -> Result<Response, TradeError> {
    // anyone can try to refund, as long as the contract is expired
    if state.expire_height > env.block.height {
        return Err(TradeError::RefundError {
            message: "Only expired trades can be refunded.".to_string(),
        });
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    return if balance_result.is_ok() {
        let balance = balance_result.unwrap();
        send_tokens(deps, state.sender, balance, "refund")
    } else {
        Err(TradeError::RefundError {
            message: "Contract has no funds.".to_string(),
        })
    };
}

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(
    deps: DepsMut,
    to_address: Addr,
    amount: Vec<Coin>,
    action: &str,
) -> Result<Response, TradeError> {
    let attributes = vec![attr("action", action), attr("to", to_address.clone())];
    let amount = [deduct_tax(&deps.querier, amount[0].clone()).unwrap()].to_vec();

    let r = Response {
        messages: vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: to_address.to_string(),
            amount,
        })],
        submessages: vec![],
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
