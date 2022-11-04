use std::ops::{Mul, Sub};
use std::str::FromStr;

use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut,
    Env, MessageInfo, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};

use localterra_protocol::constants::{ARBITRATION_FEE, LOCAL_FEE};
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::denom_utils::denom_to_string;
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::{
    FundEscrowError, HubAlreadyRegistered, InvalidTradeState, InvalidTradeStateChange,
    MissingParameter, OfferNotFound, RefundErrorNotExpired, TradeExpired,
};
use localterra_protocol::guards::{
    assert_ownership, assert_sender_is_buyer_or_seller, assert_trade_state_and_type,
    assert_trade_state_change_is_valid, assert_value_in_range,
};
use localterra_protocol::hub_utils::{get_hub_admin, get_hub_config, register_hub_internal};
use localterra_protocol::offer::{load_offer, Arbitrator, OfferType, TradeInfo};
use localterra_protocol::profile::{
    increase_profile_trades_count_msg, load_profile, update_profile_msg,
};
use localterra_protocol::trade::{
    arbitrators, ArbitratorModel, ExecuteMsg, InstantiateMsg, MigrateMsg, NewTrade, QueryMsg, Swap,
    SwapMsg, Trade, TradeModel, TradeResponse, TradeState, TradeStateItem, TraderRole,
};
use localterra_protocol::trading_incentives::ExecuteMsg as TradingIncentivesMsg;

pub const SWAP_REPLY_ID: u64 = 1u64;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new().add_attribute("action", "instantiate_trade");
    Ok(res)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
        ExecuteMsg::Create(new_trade) => create_trade(deps, env, new_trade),
        ExecuteMsg::AcceptRequest {
            trade_id,
            maker_contact,
        } => accept_request(deps, env, info, trade_id, maker_contact),
        ExecuteMsg::FundEscrow {
            trade_id,
            maker_contact,
        } => fund_escrow(deps, env, info, trade_id, maker_contact),
        ExecuteMsg::ReleaseEscrow { trade_id } => release_escrow(deps, env, info, trade_id),
        ExecuteMsg::FiatDeposited { trade_id } => fiat_deposited(deps, env, info, trade_id),
        ExecuteMsg::CancelRequest { trade_id } => cancel_request(deps, env, info, trade_id),
        ExecuteMsg::RefundEscrow { trade_id } => refund_escrow(deps, env, info, trade_id),
        ExecuteMsg::DisputeEscrow {
            trade_id,
            buyer_contact,
            seller_contact,
        } => dispute_escrow(deps, env, info, trade_id, buyer_contact, seller_contact),
        ExecuteMsg::NewArbitrator {
            arbitrator,
            fiat,
            encryption_key,
        } => create_arbitrator(deps, info, arbitrator, fiat, encryption_key),
        ExecuteMsg::DeleteArbitrator { arbitrator, fiat } => {
            delete_arbitrator(deps, info, arbitrator, fiat)
        }
        ExecuteMsg::SettleDispute { trade_id, winner } => {
            settle_dispute(deps, env, info, trade_id, winner)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn create_trade(deps: DepsMut, env: Env, new_trade: NewTrade) -> Result<Response, ContractError> {
    // Load Offer
    let hub_cfg = get_hub_config(deps.as_ref());

    let offer_id = new_trade.offer_id.clone();
    let offer_result = load_offer(
        &deps.querier,
        new_trade.offer_id.clone(),
        hub_cfg.offer_addr.to_string(),
    );
    if offer_result.is_err() {
        return Err(OfferNotFound {
            offer_id: new_trade.offer_id.to_string(),
        });
    }
    let offer_result = offer_result.unwrap();
    let offer = offer_result.offer;
    assert_value_in_range(offer.min_amount, offer.max_amount, new_trade.amount.clone()).unwrap();

    //Instantiate buyer and seller addresses according to Offer type (buy, sell)
    let buyer: Addr;
    let buyer_contact: Option<String>;
    let seller: Addr;
    let seller_contact: Option<String>;

    if offer.offer_type == OfferType::Buy {
        buyer = offer.owner.clone(); // maker
        buyer_contact = None; // maker
        seller = new_trade.taker.clone(); // taker
        seller_contact = Some(new_trade.taker_contact); // taker
    } else {
        buyer = new_trade.taker.clone(); // taker
        buyer_contact = Some(new_trade.taker_contact); // taker
        seller = offer.owner.clone(); // maker
        seller_contact = None // maker
    }

    let profile = offer_result.profile;
    let trades_count = profile.requested_trades_count + 1;
    let trade_id = [offer.id.clone(), trades_count.to_string()].join("_");

    let new_trade_state = TradeStateItem {
        actor: new_trade.taker.clone(),
        state: TradeState::RequestCreated,
        timestamp: env.block.time.seconds(),
    };
    let trade_state_history = vec![new_trade_state];

    let mut sub_msgs = vec![];
    sub_msgs.push(update_profile_msg(
        hub_cfg.profile_addr.to_string(),
        new_trade.taker.clone(),
        new_trade.profile_taker_contact,
        new_trade.profile_taker_encryption_key,
    ));

    let random_seed: u32 = (env.block.time.seconds() % 100) as u32;
    let arbitrator = ArbitratorModel::get_arbitrator_random(
        deps.as_ref(),
        random_seed as usize,
        offer.fiat_currency.clone(),
    );

    let expires_at = env.block.time.seconds() + hub_cfg.trade_expiration_timer;
    //Instantiate Trade state
    let trade = TradeModel::create(
        deps.storage,
        Trade::new(
            trade_id.clone(),
            env.contract.address.clone(),
            buyer,
            seller,
            seller_contact,
            buyer_contact,
            arbitrator.arbitrator,
            hub_cfg.offer_addr.clone(),
            offer_id,
            env.block.time.seconds(),
            expires_at,
            offer.denom.clone(),
            new_trade.amount.clone(),
            offer.fiat_currency,
            trade_state_history,
        ),
    )
    .trade;

    // increase profile profile requested_trades_count
    sub_msgs.push(increase_profile_trades_count_msg(
        hub_cfg.profile_addr.to_string(),
        profile.addr,
        TradeState::RequestCreated,
    ));

    let denom_str = denom_to_string(&trade.denom);
    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "create_trade")
        .add_attribute("trade_id", trade_id)
        .add_attribute("offer_id", offer.id.clone())
        .add_attribute("owner", offer.owner.to_string())
        .add_attribute("amount", trade.amount.to_string())
        .add_attribute("denom", denom_str)
        .add_attribute("taker", new_trade.taker.to_string());

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Trade { id } => to_binary(&query_trade(env, deps, id)?),
        QueryMsg::Trades {
            user,
            state,
            role: index,
            last_value,
            limit,
        } => to_binary(&query_trades(
            env, deps, user, state, index, last_value, limit,
        )?),
        QueryMsg::Arbitrator { arbitrator } => to_binary(&ArbitratorModel::query_arbitrator(
            deps.storage,
            arbitrator,
        )?),
        QueryMsg::Arbitrators {} => to_binary(&ArbitratorModel::query_arbitrators(deps.storage)?),
        QueryMsg::ArbitratorsFiat { fiat } => to_binary(&ArbitratorModel::query_arbitrators_fiat(
            deps.storage,
            fiat,
        )?),
    }
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_trade(env: Env, deps: Deps, id: String) -> StdResult<TradeInfo> {
    let hub_config = get_hub_config(deps);
    let state = TradeModel::from_store(deps.storage, &id);

    let buyer = load_profile(
        &deps.querier,
        hub_config.profile_addr.to_string(),
        state.buyer.clone(),
    )
    .unwrap();

    let seller = load_profile(
        &deps.querier,
        hub_config.profile_addr.to_string(),
        state.seller.clone(),
    )
    .unwrap();

    let arbitrator = load_profile(
        &deps.querier,
        hub_config.profile_addr.to_string(),
        state.arbitrator.clone(),
    )
    .unwrap();

    let block_time = env.block.time.seconds();
    let trade = TradeResponse::map(state, buyer, seller, arbitrator, block_time);
    let offer = load_offer(
        &deps.querier,
        trade.offer_id.clone(),
        hub_config.offer_addr.to_string(),
    )
    .unwrap();

    Ok(TradeInfo { trade, offer })
}

pub fn query_trades(
    env: Env,
    deps: Deps,
    user: Addr,
    _state: Option<TradeState>,
    index: TraderRole,
    last_value: Option<String>,
    limit: u32,
) -> StdResult<Vec<TradeInfo>> {
    let mut trades_infos: Vec<TradeInfo> = vec![];
    let hub_config = get_hub_config(deps);

    let trade_results = match index {
        TraderRole::Seller => {
            TradeModel::trades_by_seller(deps.storage, user.to_string(), last_value, limit).unwrap()
        }
        TraderRole::Buyer => {
            TradeModel::trades_by_buyer(deps.storage, user.to_string(), last_value, limit).unwrap()
        }
        TraderRole::Arbitrator => {
            TradeModel::trades_by_arbitrator(deps.storage, user.to_string(), last_value, limit)
                .unwrap()
        }
    };

    trade_results.iter().for_each(|trade: &Trade| {
        let offer_id = trade.offer_id.clone();
        let offer_contract = trade.offer_contract.to_string();
        let offer_response = load_offer(&deps.querier, offer_id, offer_contract).unwrap();

        // TODO change it to make only one query
        let buyer = load_profile(
            &deps.querier,
            hub_config.profile_addr.to_string(),
            trade.buyer.clone(),
        )
        .unwrap();

        let seller = load_profile(
            &deps.querier,
            hub_config.profile_addr.to_string(),
            trade.seller.clone(),
        )
        .unwrap();

        let arbitrator = load_profile(
            &deps.querier,
            hub_config.profile_addr.to_string(),
            trade.arbitrator.clone(),
        )
        .unwrap();

        let block_time = env.block.time.seconds();

        trades_infos.push(TradeInfo {
            trade: TradeResponse::map(trade.clone(), buyer, seller, arbitrator, block_time),
            offer: offer_response,
        })
    });

    Ok(trades_infos)
}

fn fund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    maker_contact: Option<String>,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);

    let offer = load_offer(
        &deps.querier.clone(),
        trade.offer_id.clone(),
        trade.offer_contract.to_string(),
    )
    .unwrap()
    .offer;

    // Everybody can set the state to RequestExpired, if it is expired (they are doing as a favor).
    if trade.request_expired(env.block.time.seconds()) {
        trade.set_state(TradeState::RequestExpired, &env, &info);
        TradeModel::store(deps.storage, &trade).unwrap();

        return Err(TradeExpired {
            expired_at: trade.expires_at,
            created_at: trade.created_at,
        });
    }

    // Only the seller wallet is authorized to fund this trade.
    assert_ownership(info.sender.clone(), trade.seller.clone()).unwrap();

    // If seller_contact is not already defined it needs to be defined here
    if trade.seller_contact.is_none() {
        if maker_contact.is_some() {
            // Set maker_contact as seller_contact
            trade.seller_contact = maker_contact
        } else {
            return Err(MissingParameter {
                missing: "maker_contact".to_string(),
                message: Some("At this point the maker_contact can not be undefined".to_string()),
            });
        }
    }

    // Ensure TradeState::Created for Sell and TradeState::Accepted for Buy orders
    assert_trade_state_and_type(&trade, &offer.offer_type).unwrap();
    let denom = denom_to_string(&trade.denom);
    let default = coin(0, denom.clone());
    let balance = info
        .funds
        .iter()
        .find(|&coin| &coin.denom == &denom)
        .unwrap_or(&default);

    // TODO: only accept exact funding amounts, return otherwise
    if balance.amount >= trade.amount {
        trade.set_state(TradeState::EscrowFunded, &env, &info);
    } else {
        return Err(FundEscrowError {
            required_amount: trade.amount.clone(),
            sent_amount: balance.amount.clone(),
        });
    }

    TradeModel::store(deps.storage, &trade).unwrap();
    let res = Response::new()
        .add_attribute("action", "fund_escrow")
        .add_attribute("trade_id", trade_id)
        .add_attribute("trade.amount", trade.amount.clone().to_string())
        .add_attribute("sent_amount", balance.amount.to_string())
        .add_attribute("seller", info.sender)
        .add_attribute("state", trade.get_state().to_string());
    Ok(res)
}

fn accept_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    maker_contact: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // Only the buyer can accept the request
    assert_ownership(info.sender.clone(), trade.buyer.clone()).unwrap();

    // Only change state if the current state is TradeState::RequestCreated
    assert_trade_state_change_is_valid(
        trade.get_state(),
        TradeState::RequestCreated,
        TradeState::RequestAccepted,
    )
    .unwrap();

    // Change trade state
    trade.set_state(TradeState::RequestAccepted, &env, &info);

    // Set maker contact as buyer
    trade.buyer_contact = Some(maker_contact);

    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "accept_request")
        .add_attribute("trade_id", trade_id)
        .add_attribute("state", trade.get_state().to_string());

    Ok(res)
}

fn fiat_deposited(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // The buyer is always the one depositing fiat
    // Only the buyer can mark the fiat as deposited
    assert_ownership(info.sender.clone(), trade.buyer.clone()).unwrap();
    assert_trade_state_change_is_valid(
        trade.get_state(),
        TradeState::EscrowFunded,
        TradeState::FiatDeposited,
    )
    .unwrap();

    // Update trade State to TradeState::FiatDeposited
    trade.set_state(TradeState::FiatDeposited, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "fiat_deposited")
        .add_attribute("trade_id", trade_id)
        .add_attribute("state", trade.get_state().to_string());

    Ok(res)
}

fn cancel_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // Only the buyer or seller can cancel the trade.
    assert_sender_is_buyer_or_seller(
        info.sender.clone(),
        trade.buyer.clone(),
        trade.seller.clone(),
    )
    .unwrap();

    // You can only cancel the trade if the current TradeState is Created or Accepted
    if !((trade.get_state() == TradeState::RequestCreated)
        || (trade.get_state() == TradeState::RequestAccepted))
    {
        return Err(InvalidTradeStateChange {
            from: trade.get_state(),
            to: TradeState::RequestCanceled,
        });
    }

    // Update trade State to TradeState::RequestCanceled
    trade.set_state(TradeState::RequestCanceled, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();
    let res = Response::new().add_attribute("action", "cancel_request");
    Ok(res)
}

fn release_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    // Load trade and validate that permission and state are valid.
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    let trade_denom = denom_to_string(&trade.denom);
    if trade.seller.eq(&info.sender) {
        assert_trade_state_change_is_valid(
            trade.get_state(),
            TradeState::FiatDeposited,
            TradeState::EscrowReleased,
        )
        .unwrap();
    } else {
        return Err(ContractError::Unauthorized {
            owner: trade.seller.clone(),
            caller: info.sender.clone(),
        });
    }

    // Load HubConfig
    let hub_cfg = get_hub_config(deps.as_ref());
    let offer = load_offer(
        &deps.querier,
        trade.offer_id.clone(),
        hub_cfg.offer_addr.to_string(),
    )
    .unwrap()
    .offer;

    // Update trade State to TradeState::EscrowReleased
    trade.set_state(TradeState::EscrowReleased, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    // Calculate fees and final release amount
    let mut send_msgs: Vec<SubMsg> = Vec::new();
    let mut release_amount = trade.amount.clone();
    let one = Uint128::new(1u128);
    let fee = one.mul(Decimal::from_ratio(release_amount, LOCAL_FEE));
    release_amount = release_amount.checked_sub(fee.clone()).unwrap();
    let burn_amount = fee.mul(Decimal::from_ratio(hub_cfg.burn_fee_pct, 100u128));
    let chain_amount = fee.mul(Decimal::from_ratio(hub_cfg.chain_fee_pct, 100u128));
    let warchest_amount = fee.mul(Decimal::from_ratio(hub_cfg.warchest_fee_pct, 100u128));

    // Create Trade Registration message to be sent to the Trading Incentives contract.
    let register_trade_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: hub_cfg.trading_incentives_addr.to_string(),
        msg: to_binary(&TradingIncentivesMsg::RegisterTrade {
            trade: trade.id.clone(),
        })
        .unwrap(),
        funds: vec![],
    }));
    send_msgs.push(register_trade_msg);

    // Update profile released_trades_count
    send_msgs.push(increase_profile_trades_count_msg(
        hub_cfg.profile_addr.to_string(),
        offer.owner.clone(),
        trade.get_state(),
    ));

    // Send tokens to buyer
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: trade.buyer.to_string(),
        amount: vec![Coin::new(release_amount.u128(), trade_denom.clone())],
    })));

    // Fee Distribution
    let local_denom = denom_to_string(&hub_cfg.local_denom);
    //If coin being traded is not $LOCAL, swap it and burn it on swap reply.
    if trade_denom.ne(&local_denom) {
        send_msgs.push(SubMsg {
            id: SWAP_REPLY_ID,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: hub_cfg.local_market_addr.to_string(),
                msg: to_binary(&SwapMsg { swap: Swap {} }).unwrap(),
                funds: vec![coin(burn_amount.u128(), trade_denom.clone())],
            }),
            gas_limit: None,
            reply_on: ReplyOn::Success,
        });
    } else {
        //If coin being traded is $LOCAL, add message burning the local_burn amount
        send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![coin(burn_amount.u128(), local_denom.clone())],
        })));
    }

    // Warchest
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: hub_cfg.warchest_addr.to_string(),
        amount: vec![coin(warchest_amount.u128(), trade_denom.clone())],
    })));

    // Chain Fee Sharing
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: hub_cfg.chain_fee_collector_addr.to_string(),
        amount: vec![coin(chain_amount.u128(), trade_denom.clone())],
    })));

    let res = Response::new()
        .add_submessages(send_msgs)
        .add_attribute("action", "release_escrow")
        .add_attribute("trade_id", trade_id.clone())
        .add_attribute("state", trade.get_state().clone().to_string());
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        SWAP_REPLY_ID => handle_swap_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_swap_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let hub_cfg = get_hub_config(deps.as_ref());
    let local_denom = denom_to_string(&hub_cfg.local_denom);
    let contract_address = &hub_cfg.trade_addr.to_string();

    let mut last_receiver = String::new();
    let mut local_amount_received = String::from("0");
    let events = msg.result.unwrap().events;
    for e in events {
        if e.ty.eq("coin_received") {
            for attr in e.attributes {
                if attr.key.eq("receiver") {
                    last_receiver = attr.value.to_string();
                } else if attr.key.eq("amount") && last_receiver.eq(contract_address) {
                    local_amount_received =
                        attr.value.to_string().replace(local_denom.as_str(), "");
                }
            }
        }
    }

    //Burn $LOCAL
    let burn_msg = CosmosMsg::Bank(BankMsg::Burn {
        amount: vec![coin(
            u128::from_str(local_amount_received.as_str()).unwrap(),
            local_denom.clone(),
        )],
    });

    let res = Response::new()
        .add_attributes(vec![
            ("event", "swap_reply"),
            ("burn_amount", local_amount_received.as_str()),
            ("denom", local_denom.as_str()),
        ])
        .add_submessage(SubMsg::new(burn_msg));
    Ok(res)
}

fn refund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    // Refund can only happen if trade state is TradeState::EscrowFunded and FundingTimeout is expired
    let trade = TradeModel::from_store(deps.storage, &trade_id);
    assert_trade_state_change_is_valid(
        trade.get_state(),
        TradeState::EscrowFunded,
        TradeState::EscrowRefunded,
    )
    .unwrap();

    // anyone can try to refund, as long as the contract is expired
    let block_time = env.block.time.seconds();
    if !trade.request_expired(block_time) {
        return Err(RefundErrorNotExpired {
            message:
                "Only expired trades that are not disputed can be refunded by non-arbitrators."
                    .to_string(),
            trade: trade.get_state().to_string(),
        });
    }

    let mut trade: Trade = TradeModel::from_store(deps.storage, &trade_id);

    //Update trade state to TradeState::EscrowRefunded
    trade.set_state(TradeState::EscrowRefunded, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    let amount = trade.amount.clone();
    let denom = denom_to_string(&trade.denom);
    let refund_amount = vec![Coin::new(amount.u128(), denom.clone())];
    let send_msg = create_send_msg(trade.seller, refund_amount);
    let res = Response::new()
        .add_attribute("action", "refund_escrow")
        .add_submessage(SubMsg::new(send_msg));
    Ok(res)
}

//region arbitration
pub fn create_arbitrator(
    deps: DepsMut,
    info: MessageInfo,
    arbitrator_address: Addr,
    fiat: FiatCurrency,
    encryption_key: String,
) -> Result<Response, ContractError> {
    let admin = get_hub_admin(deps.as_ref()).addr;
    let hub_config = get_hub_config(deps.as_ref());
    assert_ownership(info.sender, admin)?;

    ArbitratorModel::create_arbitrator(
        deps.storage,
        Arbitrator {
            arbitrator: arbitrator_address.clone(),
            fiat: fiat.clone(),
        },
    );

    let create_profile_sub_msg = update_profile_msg(
        hub_config.profile_addr.to_string(),
        arbitrator_address.clone(),
        "N/A".to_string(),
        encryption_key.clone(),
    );

    let res = Response::new()
        .add_submessage(create_profile_sub_msg)
        .add_attribute("action", "create_arbitrator")
        .add_attribute("arbitrator", arbitrator_address.to_string())
        .add_attribute("asset", fiat.to_string())
        .add_attribute("encryption_key", encryption_key);

    Ok(res)
}

pub fn delete_arbitrator(
    deps: DepsMut,
    info: MessageInfo,
    arbitrator: Addr,
    fiat: FiatCurrency,
) -> Result<Response, ContractError> {
    let admin = get_hub_admin(deps.as_ref());
    assert_ownership(info.sender, admin.addr)?;

    let index = arbitrator.clone().to_string() + &fiat.to_string();

    arbitrators().remove(deps.storage, &index).unwrap();

    let res = Response::new()
        .add_attribute("action", "delete_arbitrator")
        .add_attribute("arbitrator", arbitrator.to_string())
        .add_attribute("asset", fiat.to_string());

    Ok(res)
}

fn dispute_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    buyer_contact: String,
    seller_contact: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // TODO: check escrow funding timer*
    // Only the buyer or seller can start a dispute
    assert_sender_is_buyer_or_seller(
        info.sender.clone(),
        trade.buyer.clone(),
        trade.seller.clone(),
    )
    .unwrap();

    // Users can only start a dispute once the buyer has clicked `mark paid` after the fiat has been deposited
    assert_trade_state_change_is_valid(
        trade.get_state(),
        TradeState::FiatDeposited,
        TradeState::EscrowDisputed,
    )
    .unwrap();

    // Update trade State to TradeState::Disputed and sets arbitrator
    trade.set_state(TradeState::EscrowDisputed, &env, &info);
    trade.arbitrator_buyer_contact = Some(buyer_contact);
    trade.arbitrator_seller_contact = Some(seller_contact);
    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "dispute_escrow")
        .add_attribute("trade_id", trade.id.clone())
        .add_attribute("state", trade.get_state().to_string())
        .add_attribute("arbitrator", trade.arbitrator.to_string());

    Ok(res)
}

fn settle_dispute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
    winner: Addr,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);

    // Check if caller is the arbitrator of the given trade
    if trade.arbitrator.ne(&info.sender) {
        return Err(ContractError::Unauthorized {
            owner: trade.arbitrator.clone(),
            caller: info.sender,
        });
    }

    // Check if TradeState is EscrowDisputed
    if TradeState::EscrowDisputed.ne(&trade.get_state()) {
        return Err(InvalidTradeState {
            current: trade.get_state(),
            expected: TradeState::EscrowDisputed,
        });
    }

    // Load Offer
    let offer = load_offer(
        &deps.querier,
        trade.offer_id.clone(),
        trade.offer_contract.to_string(),
    )
    .unwrap()
    .offer;

    // Define maker and taker
    let maker = offer.owner.clone();
    let taker = if trade.seller.eq(&maker) {
        trade.buyer.clone()
    } else {
        trade.seller.clone()
    };

    // Check if winner is eligible, it must be either maker or taker
    if winner.eq(&maker) {
        trade.set_state(TradeState::SettledForMaker, &env, &info);
    } else if winner.eq(&taker) {
        trade.set_state(TradeState::SettledForTaker, &env, &info)
    } else {
        return Err(ContractError::InvalidSender {
            sender: winner,
            buyer: trade.buyer,
            seller: trade.seller,
        });
    }
    TradeModel::store(deps.storage, &trade).unwrap();

    // Pay arbitration fee
    let amount = trade.amount.clone();
    let fee_rate: Uint128 = Uint128::new(ARBITRATION_FEE);
    let fee_amount = amount.multiply_ratio(Uint128::new(1), fee_rate);

    let denom = denom_to_string(&trade.denom);
    let fee = vec![Coin::new(fee_amount.u128(), denom.clone())];
    let winner_amount = vec![Coin::new(
        amount.u128().sub(fee_amount.u128()),
        denom.clone(),
    )];

    let winner_msg = create_send_msg(winner.clone(), winner_amount);
    let arbitrator_msg = create_send_msg(trade.arbitrator.clone(), fee);

    let res = Response::new()
        .add_attribute("arbitrator", trade.arbitrator.to_string())
        .add_attribute("winner", winner.to_string())
        .add_attribute("maker", maker.to_string())
        .add_attribute("taker", taker.to_string())
        .add_submessage(SubMsg::new(winner_msg))
        .add_submessage(SubMsg::new(arbitrator_msg));
    Ok(res)
}

// region utils
pub fn get_fee_amount(amount: Uint128, fee: u128) -> Uint128 {
    amount.clone().checked_div(Uint128::new(fee)).unwrap() // TODO: use constant / config
}

fn create_send_msg(to_address: Addr, amount: Vec<Coin>) -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: to_address.to_string(),
        amount,
    })
}
//endregion
