use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, ContractResult, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, QueryRequest, Reply, ReplyOn, Response, StdError, StdResult, Storage, SubMsg,
    SubMsgExecutionResponse, Uint128, WasmMsg, WasmQuery,
};
use cosmwasm_storage::{bucket, bucket_read};

use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::factory_util::get_factory_config;
use localterra_protocol::offer::{
    Config, ExecuteMsg, InstantiateMsg, Offer, OfferMsg, OfferState, QueryMsg, State, TradeInfo,
    OFFERS,
};
use localterra_protocol::trade::{
    InstantiateMsg as TradeInstantiateMsg, QueryMsg as TradeQueryMsg, State as TradeState,
};

use crate::errors::OfferError;
use crate::state::{
    config_read, config_storage, query_all_offers, query_all_trades, state_read, state_storage,
    OFFERS_KEY, TRADES,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, OfferError> {
    config_storage(deps.storage).save(&Config {
        factory_addr: info.sender,
    })?;
    state_storage(deps.storage).save(&State { offers_count: 0 })?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, OfferError> {
    match msg {
        ExecuteMsg::Create { offer } => create_offer(deps, env, info, offer),
        ExecuteMsg::Activate { id } => activate_offer(deps, env, info, id),
        ExecuteMsg::Pause { id } => pause_offer(deps, env, info, id),
        ExecuteMsg::Update { id, offer } => update_offer(deps, env, info, id, offer),
        ExecuteMsg::NewTrade {
            offer_id,
            ust_amount,
            counterparty,
        } => create_trade(deps, env, info, offer_id, ust_amount, counterparty),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offers { fiat_currency } => to_binary(&load_offers(deps.storage, fiat_currency)?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps.storage, id)?),
        QueryMsg::Trades { maker } => to_binary(&load_trades(
            deps,
            deps.api.addr_validate(maker.as_str()).unwrap(),
        )?),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, OfferError> {
    match msg.id {
        0 => trade_instance_reply(deps, env, msg.result),
        _ => Err(OfferError::InvalidReply {}),
    }
}

fn trade_instance_reply(
    deps: DepsMut,
    _env: Env,
    result: ContractResult<SubMsgExecutionResponse>,
) -> Result<Response, OfferError> {
    if result.is_err() {
        return Err(OfferError::InvalidReply {});
    }

    let trade_addr: Addr = result
        .unwrap()
        .events
        .into_iter()
        .find(|e| e.ty == "instantiate_contract")
        .and_then(|ev| {
            ev.attributes
                .into_iter()
                .find(|attr| attr.key == "contract_address")
                .map(|addr| addr.value)
        })
        .and_then(|addr| deps.api.addr_validate(addr.as_str()).ok())
        .unwrap();

    let trade_state: TradeState = deps
        .querier
        .query_wasm_smart(trade_addr.to_string(), &TradeQueryMsg::State {})
        .unwrap();

    let offer = load_offer_by_id(deps.storage, trade_state.offer_id.clone()).unwrap();

    let trades_store = TRADES.key(offer.owner.as_bytes());
    let mut trades = trades_store.load(deps.storage).unwrap_or(vec![]);
    trades.push(trade_addr.clone());
    trades_store.save(deps.storage, &trades).unwrap();

    //trade_state, offer_id, trade_amount,owner
    let res = Response::new()
        .add_attribute("action", "create_trade_reply")
        .add_attribute("addr", trade_addr)
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("amount", trade_state.ust_amount)
        .add_attribute("owner", offer.owner);
    Ok(res)
}

pub fn create_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, OfferError> {
    if msg.min_amount >= msg.max_amount {
        let err = OfferError::Std(StdError::generic_err(
            "Min amount must be greater than Max amount.",
        ));
        return Err(err);
    }

    let mut state = state_storage(deps.storage).load()?;

    let offer_id = state.offers_count + 1;

    state.offers_count = offer_id;

    let offer = Offer {
        id: offer_id,
        owner: info.sender.clone(),
        offer_type: msg.offer_type,
        fiat_currency: msg.fiat_currency.clone(),
        min_amount: Uint128::from(msg.min_amount),
        max_amount: Uint128::from(msg.max_amount),
        state: OfferState::Active,
    };

    offer.save(deps.storage)?;

    state_storage(deps.storage).save(&state)?;

    let res = Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("type", offer.offer_type.to_string())
        .add_attribute("id", offer.id.to_string())
        .add_attribute("min_amount", offer.min_amount.to_string())
        .add_attribute("max_amount", offer.max_amount.to_string())
        .add_attribute("owner", offer.owner);

    Ok(res)
}

pub fn activate_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, OfferError> {
    let mut offer = OFFERS
        .may_load(deps.storage, &id.to_be_bytes())?
        .ok_or(OfferError::InvalidReply {})?;

    if !(offer.owner.eq(&info.sender)) {
        return Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        });
    }

    match offer.state {
        OfferState::Paused => {
            offer.activate(deps.storage);

            let res = Response::new()
                .add_attribute("action", "activate_offer")
                .add_attribute("id", offer.id.to_string())
                .add_attribute("owner", offer.owner.to_string());

            Ok(res)
        }
        OfferState::Active => Err(OfferError::InvalidStateChange {
            from: offer.state,
            to: OfferState::Active,
        }),
    }
}

pub fn pause_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
) -> Result<Response, OfferError> {
    let mut offer = OFFERS
        .may_load(deps.storage, &id.to_be_bytes())?
        .ok_or(OfferError::InvalidReply {})?;

    if !(offer.owner.eq(&info.sender)) {
        return Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        });
    }

    match offer.state {
        OfferState::Active => {
            offer.pause(deps.storage);

            let res = Response::new()
                .add_attribute("action", "pause_offer")
                .add_attribute("id", offer.id.to_string())
                .add_attribute("owner", offer.owner.to_string());

            Ok(res)
        }
        OfferState::Paused => Err(OfferError::InvalidStateChange {
            from: offer.state,
            to: OfferState::Paused,
        }),
    }
}

pub fn update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    msg: OfferMsg,
) -> Result<Response, OfferError> {
    let mut offer = OFFERS
        .may_load(deps.storage, &id.to_be_bytes())?
        .ok_or(OfferError::InvalidReply {})?;

    if msg.min_amount >= msg.max_amount {
        let err = OfferError::Std(StdError::generic_err(
            "Min amount must be greater than Max amount.",
        ));
        return Err(err);
    }

    if !(offer.owner.eq(&info.sender)) {
        Err(OfferError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        })
    } else {
        offer.update(deps.storage, msg);

        let res = Response::new()
            .add_attribute("action", "pause_offer")
            .add_attribute("id", offer.id.to_string())
            .add_attribute("owner", offer.owner.to_string());

        Ok(res)
    }
}

fn create_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    offer_id: u64,
    ust_amount: String,
    counterparty: String,
) -> Result<Response, OfferError> {
    let cfg = config_read(deps.storage).load().unwrap();
    // let offer = load_offer_by_id(deps.storage, offer_id).unwrap();
    let offer = OFFERS
        .may_load(deps.storage, &offer_id.to_be_bytes())?
        .ok_or(OfferError::InvalidReply {})?; // TODO choose better error

    //TODO: Discuss this with the team.
    /*
    if info.sender.ne(&offer.owner) {
        return Err(OfferError::Unauthorized {
            owner: offer.owner.clone(),
            caller: info.sender.clone(),
        });
    }
    */

    let factory_cfg = get_factory_config(&deps.querier, cfg.factory_addr.to_string());

    let instantiate_msg = WasmMsg::Instantiate {
        admin: None,
        code_id: factory_cfg.trade_code_id,
        msg: to_binary(&TradeInstantiateMsg {
            offer_id,
            ust_amount: ust_amount.clone(),
            counterparty: counterparty.clone(),
            offers_addr: env.contract.address.to_string(),
        })
        .unwrap(),
        funds: info.funds,
        label: "new-trade".to_string(),
    };
    let sub_message = SubMsg {
        id: 0,
        msg: CosmosMsg::Wasm(instantiate_msg),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    let res = Response::new()
        .add_submessage(sub_message)
        .add_attribute("action", "create_trade")
        .add_attribute("id", offer.id.to_string())
        .add_attribute("owner", offer.owner.to_string())
        .add_attribute("ust_amount", ust_amount)
        .add_attribute("counterparty", counterparty);
    Ok(res)
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let cfg = config_read(deps.storage).load().unwrap();
    Ok(cfg)
}

fn query_state(deps: Deps) -> StdResult<State> {
    let state = state_read(deps.storage).load().unwrap();
    Ok(state)
}

pub fn load_offers(storage: &dyn Storage, fiat_currency: FiatCurrency) -> StdResult<Vec<Offer>> {
    let offers = query_all_offers(storage, fiat_currency)?;
    Ok(offers)
}

pub fn load_offer_by_id(storage: &dyn Storage, id: u64) -> StdResult<Offer> {
    let offer: Offer = bucket_read(storage, OFFERS_KEY)
        .load(&id.to_be_bytes())
        .unwrap();
    Ok(offer)
}

pub fn load_trades(deps: Deps, maker: Addr) -> StdResult<Vec<TradeInfo>> {
    let trades = query_all_trades(deps.storage, maker.clone()).unwrap_or(vec![]);
    let mut trades_infos: Vec<TradeInfo> = vec![];
    trades.iter().for_each(|t| {
        let trade_state: TradeState = deps
            .querier
            .query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: t.to_string(),
                msg: to_binary(&TradeQueryMsg::State {}).unwrap(),
            }))
            .unwrap();
        let offer: Offer = deps
            .querier
            .query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: trade_state.offer_contract.to_string(),
                msg: to_binary(&QueryMsg::Offer {
                    id: trade_state.offer_id,
                })
                .unwrap(),
            }))
            .unwrap();
        trades_infos.push(TradeInfo {
            trade: trade_state,
            offer,
        })
    });
    Ok(trades_infos)
}
