use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut,
    Env, MessageInfo, QueryRequest, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128,
    WasmMsg, WasmQuery,
};
use std::ops::Mul;
use std::str::FromStr;

use localterra_protocol::constants::{FUNDING_TIMEOUT, LOCAL_FEE, REQUEST_TIMEOUT};
use localterra_protocol::denom_utils::denom_to_string;
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::{
    FundEscrowError, HubAlreadyRegistered, InvalidTradeStateChange, OfferNotFound,
    RefundErrorNotExpired, TradeExpired,
};
use localterra_protocol::guards::{
    assert_ownership, assert_sender_is_buyer_or_seller, assert_trade_state_and_type,
    assert_trade_state_change_is_valid, assert_value_in_range, trade_request_is_expired,
};
use localterra_protocol::hub_utils::{get_hub_config, register_hub_internal};
use localterra_protocol::offer::ExecuteMsg::{UpdateLastTraded};
use localterra_protocol::offer::{
    load_offer, Arbitrator, Offer, OfferType, QueryMsg as OfferQueryMsg, TradeInfo,
};
use localterra_protocol::trade::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, NewTrade, QueryMsg, Swap, SwapMsg, Trade, TradeModel,
    TradeState, TraderRole,
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
        ExecuteMsg::Create(new_trade) => create_trade(deps, env, new_trade),
        ExecuteMsg::AcceptRequest { trade_id } => accept_request(deps, env, info, trade_id),
        ExecuteMsg::FundEscrow { trade_id } => fund_escrow(deps, env, info, trade_id),
        ExecuteMsg::ReleaseEscrow { trade_id } => release_escrow(deps, info, trade_id),
        ExecuteMsg::FiatDeposited { trade_id } => fiat_deposited(deps, info, trade_id),
        ExecuteMsg::CancelRequest { trade_id } => cancel_request(deps, info, trade_id),
        ExecuteMsg::RefundEscrow { trade_id } => refund_escrow(deps, env, trade_id),
        ExecuteMsg::DisputeEscrow { trade_id } => dispute_escrow(deps, env, info, trade_id),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

fn create_trade(deps: DepsMut, env: Env, new_trade: NewTrade) -> Result<Response, ContractError> {
    //Load Offer
    let hub_cfg = get_hub_config(deps.as_ref());

    let offer_id = new_trade.offer_id.clone();
    let offer = load_offer(
        &deps.querier,
        new_trade.offer_id.clone(),
        hub_cfg.offer_addr.to_string(),
    );
    if offer.is_none() {
        return Err(OfferNotFound {
            offer_id: new_trade.offer_id.to_string(),
        });
    }
    let offer = offer.unwrap();
    assert_value_in_range(offer.min_amount, offer.max_amount, new_trade.amount.clone()).unwrap();

    //Instantiate buyer and seller addresses according to Offer type (buy, sell)
    let buyer: Addr;
    let seller: Addr;

    if offer.offer_type == OfferType::Buy {
        buyer = offer.owner.clone(); // maker
        seller = new_trade.taker.clone(); // taker
    } else {
        buyer = new_trade.taker.clone(); // taker
        seller = offer.owner.clone(); // maker
    }

    let trade_count = offer.trades_count + 1;
    let trade_id = [offer.id.clone(), trade_count.to_string()].join("_");

    //Instantiate Trade state
    let trade = TradeModel::create(
        deps.storage,
        Trade {
            id: trade_id.clone(),
            addr: env.contract.address.clone(),
            buyer,  // buyer
            seller, // seller
            offer_contract: hub_cfg.offer_addr.clone(),
            offer_id,
            arbitrator: Some(Addr::unchecked("todo")),
            state: TradeState::RequestCreated,
            created_at: env.block.time.seconds(),
            denom: offer.denom.clone(),
            amount: new_trade.amount.clone(),
            asset: offer.fiat_currency,
        },
    )
    .trade;

    //SubMsg to Offer to contract increment trades count.
    let increment_submsg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: hub_cfg.offer_addr.to_string(),
        msg: to_binary(
            &localterra_protocol::offer::ExecuteMsg::IncrementTradesCount {
                offer_id: offer.id.clone(),
            },
        )
        .unwrap(),
        funds: vec![],
    }));

    let denom_str = denom_to_string(&trade.denom);
    let res = Response::new()
        .add_attribute("action", "create_trade")
        .add_submessage(increment_submsg)
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
        QueryMsg::Trade { id } => to_binary(&query_trade(deps, id)?),
        QueryMsg::Trades {
            user,
            state,
            role: index,
            last_value,
            limit,
        } => to_binary(&query_trades(
            env, deps, user, state, index, last_value, limit,
        )?),
    }
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_trade(deps: Deps, id: String) -> StdResult<Trade> {
    let state = TradeModel::from_store(deps.storage, &id);
    Ok(state)
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

    let trade_results = match index {
        TraderRole::Seller => {
            TradeModel::trades_by_seller(deps.storage, user.to_string(), last_value, limit).unwrap()
        }
        TraderRole::Buyer => {
            TradeModel::trades_by_buyer(deps.storage, user.to_string(), last_value, limit).unwrap()
        }
    };

    trade_results.iter().for_each(|trade: &Trade| {
        let offer_id = trade.offer_id.clone();
        let offer_contract = trade.offer_contract.to_string();
        let offer: Offer = load_offer(&deps.querier, offer_id, offer_contract).unwrap();
        let current_time = env.block.time.seconds();
        let expired = current_time > trade.created_at + REQUEST_TIMEOUT;
        trades_infos.push(TradeInfo {
            trade: trade.clone(),
            offer,
            expired,
        })
    });

    Ok(trades_infos)
}

fn fund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);

    let offer = load_offer(
        &deps.querier.clone(),
        trade.offer_id.clone(),
        trade.offer_contract.to_string(),
    )
    .unwrap();

    // Everybody can set the state to RequestExpired, if it is expired (they are doing as a favor).
    if trade_request_is_expired(env.block.time.seconds(), trade.created_at, REQUEST_TIMEOUT) {
        trade.state = TradeState::RequestExpired;
        TradeModel::store(deps.storage, &trade).unwrap();

        return Err(TradeExpired {
            timeout: REQUEST_TIMEOUT,
            expired_at: env.block.time.seconds() + REQUEST_TIMEOUT,
            created_at: trade.created_at,
        });
    }

    // Only the seller wallet is authorized to fund this trade.
    assert_ownership(info.sender.clone(), trade.seller.clone()).unwrap();

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
        trade.state = TradeState::EscrowFunded;
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
        .add_attribute("state", trade.state.to_string());
    Ok(res)
}

fn dispute_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // TODO: check escrow funding timer*
    // Only the buyer or seller can start a dispute
    assert_sender_is_buyer_or_seller(info.sender.clone(), trade.buyer.clone(), trade.seller.clone())
        .unwrap();

    // Users can only start a dispute once the buyer has clicked `mark paid` after the fiat has been deposited
    assert_trade_state_change_is_valid(
        trade.state,
        TradeState::FiatDeposited,
        TradeState::EscrowDisputed,
    )
    .unwrap();

    // Update trade State to TradeState::Disputed
    trade.state = TradeState::EscrowDisputed;

    /*
    // Assign a pseudo random arbitrator to the trade
    let arbitrator: Arbitrator = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: trade.offer_contract.clone().to_string(),
            msg: to_binary(&OfferQueryMsg::ArbitratorRandom {
                random_value: (env.block.time.seconds() % 100) as u32, // Generates a range of 0..99
                asset: trade.asset.clone(),
            })
            .unwrap(),
        }))
        .unwrap();
     */

    trade.arbitrator = Some(info.sender.clone());
    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "dispute_escrow")
        .add_attribute("trade_id", trade.id.clone())
        .add_attribute("state", trade.state.to_string())
        .add_attribute("arbitrator", trade.arbitrator.unwrap().to_string());

    Ok(res)
}

fn accept_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // Only the buyer can accept the request
    assert_ownership(info.sender, trade.buyer.clone()).unwrap();

    // Only change state if the current state is TradeState::RequestCreated
    assert_trade_state_change_is_valid(
        trade.state.clone(),
        TradeState::RequestCreated,
        TradeState::RequestAccepted,
    )
    .unwrap();

    trade.state = TradeState::RequestAccepted;

    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "accept_request")
        .add_attribute("trade_id", trade_id)
        .add_attribute("state", trade.state.to_string());

    Ok(res)
}

fn fiat_deposited(
    deps: DepsMut,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // The buyer is always the one depositing fiat
    // Only the buyer can mark the fiat as deposited
    assert_ownership(info.sender, trade.buyer.clone()).unwrap();
    assert_trade_state_change_is_valid(
        trade.state.clone(),
        TradeState::EscrowFunded,
        TradeState::FiatDeposited,
    )
    .unwrap();

    // Update trade State to TradeState::FiatDeposited
    trade.state = TradeState::FiatDeposited;

    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "fiat_deposited")
        .add_attribute("trade_id", trade_id)
        .add_attribute("state", trade.state.to_string());

    Ok(res)
}

fn cancel_request(
    deps: DepsMut,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    // Only the buyer or seller can cancel the trade.
    assert_sender_is_buyer_or_seller(info.sender, trade.buyer.clone(), trade.seller.clone())
        .unwrap();

    // You can only cancel the trade if the current TradeState is Created or Accepted
    if !((trade.state == TradeState::RequestCreated)
        || (trade.state == TradeState::RequestAccepted))
    {
        return Err(InvalidTradeStateChange {
            from: trade.state,
            to: TradeState::RequestCanceled,
        });
    }

    // Update trade State to TradeState::RequestCanceled
    trade.state = TradeState::RequestCanceled;
    TradeModel::store(deps.storage, &trade).unwrap();
    let res = Response::new().add_attribute("action", "cancel_request");
    Ok(res)
}

fn release_escrow(
    deps: DepsMut,
    info: MessageInfo,
    trade_id: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, &trade_id);
    let trade_denom = denom_to_string(&trade.denom);
    if trade.seller.eq(&info.sender) {
        assert_trade_state_change_is_valid(
            trade.state.clone(),
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

    let hub_cfg = get_hub_config(deps.as_ref());
    let offer = load_offer(
        &deps.querier,
        trade.offer_id.clone(),
        hub_cfg.offer_addr.to_string(),
    )
    .unwrap();

    //Update trade State to TradeState::EscrowReleased
    trade.state = TradeState::EscrowReleased;
    TradeModel::store(deps.storage, &trade).unwrap();

    //Calculate fees and final release amount
    let mut send_msgs: Vec<SubMsg> = Vec::new();
    let mut release_amount = trade.amount.clone();
    let one = Uint128::new(1u128);
    let fee = one.mul(Decimal::from_ratio(release_amount, LOCAL_FEE));
    release_amount = release_amount.checked_sub(fee.clone()).unwrap();
    let burn_amount = fee.mul(Decimal::from_ratio(hub_cfg.burn_fee_pct, 100u128));
    let chain_amount = fee.mul(Decimal::from_ratio(hub_cfg.chain_fee_pct, 100u128));
    let warchest_amount = fee.mul(Decimal::from_ratio(hub_cfg.warchest_fee_pct, 100u128));

    //Create Trade Registration message to be sent to the Trading Incentives contract.
    let register_trade_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: hub_cfg.trading_incentives_addr.to_string(),
        msg: to_binary(&TradingIncentivesMsg::RegisterTrade {
            trade: trade.id.clone(),
        })
        .unwrap(),
        funds: vec![],
    }));
    send_msgs.push(register_trade_msg);

    // Update the last_traded_at timestamp in the offer, so we can filter out stale ones on the user side
    let update_last_traded_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: hub_cfg.offer_addr.to_string(),
        msg: to_binary(&UpdateLastTraded { offer_id: offer.id }).unwrap(),
        funds: vec![],
    }));
    send_msgs.push(update_last_traded_msg);

    // Send tokens to buyer
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: trade.buyer.into_string(),
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
        .add_attribute("trade_id", trade_id)
        .add_attribute("state", trade.state.to_string());
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

fn refund_escrow(deps: DepsMut, env: Env, trade_id: String) -> Result<Response, ContractError> {
    // Refund can only happen if trade state is TradeState::EscrowFunded and FundingTimeout is expired
    let trade = TradeModel::from_store(deps.storage, &trade_id);
    assert_trade_state_change_is_valid(
        trade.state.clone(),
        TradeState::EscrowFunded,
        TradeState::EscrowRefunded,
    )
    .unwrap();

    // anyone can try to refund, as long as the contract is expired
    // no one except arbitrator can refund if the trade is in arbitration
    let expired = env.block.time.seconds() > trade.created_at + FUNDING_TIMEOUT;
    if !expired {
        return Err(RefundErrorNotExpired {
            message:
                "Only expired trades that are not disputed can be refunded by non-arbitrators."
                    .to_string(),
            trade: trade.state.to_string(),
        });
    }

    //Update trade state to TradeState::EscrowRefunded
    let mut trade: Trade = TradeModel::from_store(deps.storage, &trade_id);
    trade.state = TradeState::EscrowRefunded;
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

fn _settle_dispute() -> () {
    /*
    // The arbitrator can only release the escrow if the trade.state is EscrowDisputed
    let arbitration_mode =
        (info.sender.clone() == arbitrator) & (trade.state == TradeState::EscrowDisputed);

    if (offer.offer_type == OfferType::Buy) & (offer.owner == trade.buyer) {
        trade.state = TradeState::SettledForMaker;
    } else {
        trade.state = TradeState::SettledForTaker;
    }
    // Pay arbitration fee
    if arbitration_mode {
        let fee_rate: Uint128 = Uint128::new(10);
        let fee_amount = amount.multiply_ratio(Uint128::new(1), fee_rate);

        let fee = vec![Coin::new(fee_amount.u128(), denom.clone())];
        let seller_amount = vec![Coin::new(amount.sub(fee_amount).u128(), denom.clone())];

        let seller_msg = create_send_msg(trade.seller, seller_amount);
        let arbitrator_msg = create_send_msg(trade.arbitrator.clone().unwrap(), fee);

        let res = Response::new()
            .add_submessage(SubMsg::new(seller_msg))
            .add_submessage(SubMsg::new(arbitrator_msg));
        Ok(res)
    }
    //Arbitration Fee
    if arbitration_mode {
        let arbitration_amount = get_fee_amount(trade.amount.clone(), ARBITRATOR_FEE);
        release_amount -= arbitration_amount;

        // Send arbitration fee share
        send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: arbitrator.to_string(),
            amount: vec![],
        })));
    }
    */
}

pub fn get_fee_amount(amount: Uint128, fee: u128) -> Uint128 {
    amount.clone().checked_div(Uint128::new(fee)).unwrap() // TODO: use constant / config
}

fn create_send_msg(to_address: Addr, amount: Vec<Coin>) -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: to_address.to_string(),
        amount,
    })
}
