use std::ops::{Add, Sub};

use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, QueryRequest, ReplyOn, Response, StdResult, SubMsg, Uint128,
    WasmMsg, WasmQuery,
};

use localterra_protocol::constants::{FUNDING_TIMEOUT, REQUEST_TIMEOUT};
use localterra_protocol::factory::Config as FactoryConfig;
use localterra_protocol::factory_util::get_factory_config;
use localterra_protocol::guards::{
    assert_caller_is_buyer_or_seller, assert_caller_is_seller_or_arbitrator, assert_ownership,
    assert_trade_state_and_type, assert_trade_state_change_is_valid, assert_value_in_range,
    trade_request_is_expired,
};
use localterra_protocol::offer::ExecuteMsg::UpdateTradeArbitrator;
use localterra_protocol::offer::{
    Arbitrator, Config as OfferConfig, Offer, OfferType, QueryMsg as OfferQueryMsg,
};
use localterra_protocol::trade::{ExecuteMsg, InstantiateMsg, QueryMsg, TradeData, TradeState};
use localterra_protocol::trading_incentives::ExecuteMsg as TradingIncentivesMsg;

use crate::errors::TradeError;
use crate::state::{state as state_storage, state_read};
use crate::taxation::{compute_tax, deduct_tax};

const EXECUTE_UPDATE_TRADE_ARBITRATOR_REPLY_ID: u64 = 0u64;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, TradeError> {
    //Load Offer
    let offer_contract = deps.api.addr_validate(msg.offers_addr.as_str()).unwrap();
    let offer_id = msg.offer_id;
    let offer = load_offer(deps.querier, msg.offer_id, offer_contract.to_string());
    if offer.is_none() {
        return Err(TradeError::OfferNotFound {
            offer_id: msg.offer_id,
        });
    }
    let offer = offer.unwrap();

    //Load Offer Contract Config
    let load_offer_config_result: StdResult<OfferConfig> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: offer_contract.clone().into_string(),
            msg: to_binary(&OfferQueryMsg::Config {}).unwrap(),
        }));
    let offers_cfg = load_offer_config_result.unwrap();

    assert_value_in_range(offer.min_amount, offer.max_amount, msg.ust_amount).unwrap(); // TODO test this guard

    //Instantiate buyer and seller addresses according to Offer type (buy, sell)
    let buyer: Addr;
    let seller: Addr;
    let taker = deps.api.addr_validate(msg.taker.as_str()).unwrap();

    if offer.offer_type == OfferType::Buy {
        buyer = offer.owner; // maker
        seller = taker.clone(); // taker
    } else {
        buyer = taker.clone(); // taker
        seller = offer.owner; // maker
    }

    //Instantiate Trade state
    let trade = TradeData {
        addr: env.contract.address.clone(),
        factory_addr: offers_cfg.factory_addr.clone(),
        buyer,  // buyer
        seller, // seller
        offer_contract: offer_contract.clone(),
        offer_id,
        taker_contact: msg.taker_contact,
        arbitrator: None,
        state: TradeState::RequestCreated,
        created_at: env.block.time.seconds(),
        ust_amount: msg.ust_amount,
        asset: offer.fiat_currency,
    };

    //Save state.
    let save_state_result = state_storage(deps.storage).save(&trade);
    if save_state_result.is_err() {
        return Err(TradeError::InstantiationError {
            message: "Couldn't save state.".to_string(),
        });
    }

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, TradeError> {
    let state = state_storage(deps.storage).load().unwrap();
    match msg {
        ExecuteMsg::FundEscrow {} => fund_escrow(deps, env, info, state),
        ExecuteMsg::RefundEscrow {} => refund_escrow(deps, env, info, state),
        ExecuteMsg::ReleaseEscrow {} => release_escrow(deps, env, info, state),
        ExecuteMsg::DisputeEscrow {} => dispute_escrow(deps, env, info, state),
        ExecuteMsg::AcceptRequest {} => accept_request(deps, env, info, state),
        ExecuteMsg::FiatDeposited {} => fiat_deposited(deps, env, info, state),
        ExecuteMsg::CancelRequest {} => cancel_request(deps, env, info, state),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

fn query_state(deps: Deps) -> StdResult<TradeData> {
    let state = state_read(deps.storage).load()?;
    Ok(state)
}

fn load_offer(querier: QuerierWrapper, offer_id: u64, offer_contract: String) -> Option<Offer> {
    let load_offer_result: StdResult<Offer> =
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: offer_contract.clone(),
            msg: to_binary(&OfferQueryMsg::Offer { id: offer_id }).unwrap(),
        }));

    if load_offer_result.is_err() {
        None
    } else {
        Some(load_offer_result.unwrap())
    }
}

fn fund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut trade: TradeData,
) -> Result<Response, TradeError> {
    let offer = load_offer(
        deps.querier.clone(),
        trade.offer_id,
        trade.offer_contract.to_string(),
    )
    .unwrap(); //at this stage, offer is guaranteed to exists.

    // Everybody can set the state to RequestExpired, if it is expired (they are doing as a favor).
    // TODO write test for RequestExpired, attempt to fund
    if trade_request_is_expired(env.block.time.seconds(), trade.created_at, REQUEST_TIMEOUT) {
        trade.state = TradeState::RequestExpired;

        state_storage(deps.storage).save(&trade).unwrap();

        return Err(TradeError::Expired {
            timeout: REQUEST_TIMEOUT,
            expired_at: env.block.time.seconds() + REQUEST_TIMEOUT,
            created_at: trade.created_at,
        });
    }

    // Only the seller wallet is authorized to fund this trade.
    assert_ownership(info.sender.clone(), trade.seller.clone()).unwrap(); // TODO test this case

    // Ensure TradeState::Created for Sell and TradeState::Accepted for Buy orders
    assert_trade_state_and_type(&trade, &offer.offer_type).unwrap(); // TODO test this case

    // TODO only accept exact funding amounts, return otherwise
    let sent_ust_amount = if !info.funds.is_empty() {
        get_ust_amount(info.clone())
    } else {
        let ust_balance = deps
            .querier
            .query_balance(env.contract.address, "uusd".to_string());
        ust_balance
            .unwrap_or(Coin {
                denom: "uusd".to_string(),
                amount: Uint128::zero(),
            })
            .amount
    };

    if sent_ust_amount >= trade.ust_amount {
        trade.state = TradeState::EscrowFunded;
    } else {
        return Err(TradeError::FundEscrowError {
            required_amount: trade.ust_amount.clone(),
            sent_amount: sent_ust_amount.clone(),
        });
    }

    state_storage(deps.storage).save(&trade).unwrap();
    let res = Response::new()
        .add_attribute("action", "fund_escrow")
        .add_attribute("trade.ust_amount", trade.ust_amount.to_string())
        .add_attribute("sent_ust_amount", sent_ust_amount.to_string())
        .add_attribute("seller", info.sender)
        .add_attribute("state", trade.state.to_string());

    Ok(res)
}

fn get_offer(deps: &Deps, state: &TradeData) -> Offer {
    deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.offer_contract.to_string(),
            msg: to_binary(&OfferQueryMsg::Offer {
                id: state.offer_id.clone(),
            })
            .unwrap(),
        }))
        .unwrap()
}

fn dispute_escrow(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut trade: TradeData,
) -> Result<Response, TradeError> {
    // TODO check escrow funding timer
    // Only the buyer or seller can start a dispute
    assert_caller_is_buyer_or_seller(info.sender, trade.buyer.clone(), trade.seller.clone())
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

    // Assign a pseudo random arbitrator to the trade
    // TODO this needs to update the TradeAddr::arbitrator field in the trades() indexedmap of the offer contract
    let arbitrator: Arbitrator = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: trade.offer_contract.clone().to_string(),
        msg: to_binary(&OfferQueryMsg::ArbitratorRandom {
            random_value: (_env.block.time.seconds() % 100) as u32, // Generates a range of 0..99
            asset: trade.asset.clone(),
        })
        .unwrap(),
    }))?;

    trade.arbitrator = Some(arbitrator.arbitrator);

    state_storage(deps.storage).save(&trade).unwrap();
    // Update TradeAddr::Arbitrator in offer contract storage to enable querying by arbirator
    let execute_msg = WasmMsg::Execute {
        contract_addr: trade.offer_contract.to_string(),
        funds: vec![],
        msg: to_binary(&UpdateTradeArbitrator {
            arbitrator: trade.arbitrator.clone().unwrap(),
        })
        .unwrap(),
    };
    let sub_message = SubMsg {
        id: EXECUTE_UPDATE_TRADE_ARBITRATOR_REPLY_ID,
        msg: CosmosMsg::Wasm(execute_msg),
        gas_limit: None,
        reply_on: ReplyOn::Never, // TODO should we throw an error if the execution fails ?
    };

    let res = Response::new()
        .add_submessage(sub_message)
        .add_attribute("state", trade.state.to_string())
        .add_attribute("arbitrator", trade.arbitrator.unwrap().to_string());

    Ok(res)
}

fn accept_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut trade: TradeData,
) -> Result<Response, TradeError> {
    // Only the buyer can accept the request
    assert_ownership(info.sender, trade.buyer.clone()).unwrap(); // TODO test this case

    // Only change state if the current state is TradeState::RequestCreated
    assert_trade_state_change_is_valid(
        trade.state.clone(),
        TradeState::RequestCreated,
        TradeState::RequestAccepted,
    )
    .unwrap(); // TODO test this case

    trade.state = TradeState::RequestAccepted;

    state_storage(deps.storage).save(&trade).unwrap();

    let res = Response::new().add_attribute("state", trade.state.to_string());

    Ok(res)
}

fn fiat_deposited(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mut trade: TradeData,
) -> Result<Response, TradeError> {
    // The buyer is always the one depositing fiat
    // Only the buyer can mark the fiat as deposited
    assert_ownership(info.sender, trade.buyer.clone()).unwrap(); // TODO test this case
    assert_trade_state_change_is_valid(
        trade.state.clone(),
        TradeState::EscrowFunded,
        TradeState::FiatDeposited,
    )
    .unwrap(); // TODO test this case

    // Update trade State to TradeState::FiatDeposited
    trade.state = TradeState::FiatDeposited;

    state_storage(deps.storage).save(&trade).unwrap();

    let res = Response::new().add_attribute("state", trade.state.to_string());

    Ok(res)
}

fn cancel_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    state: TradeData,
) -> Result<Response, TradeError> {
    // TODO anyone can set the state to RequestExpired

    // Only the buyer or seller can cancel the trade.
    assert_caller_is_buyer_or_seller(info.sender, state.buyer, state.seller).unwrap(); // TODO test this case

    // Update trade State to TradeState::RequestCanceled
    let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

    trade.state = TradeState::RequestCanceled;

    state_storage(deps.storage).save(&trade).unwrap();

    let res = Response::new();

    Ok(res)
}

fn release_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade: TradeData,
) -> Result<Response, TradeError> {
    let arbitrator = match trade.arbitrator.clone() {
        Some(one) => one,
        None => Addr::unchecked(""), // So we can compare Addr Types
    };
    // Only seller and arbitrator can release the escrow
    assert_caller_is_seller_or_arbitrator(
        info.sender.clone(),
        trade.seller.clone(),
        arbitrator.clone(),
    )
    .unwrap();

    // The seller can only release the escrow if the trade.state is FiatDeposited
    if &info.sender == &trade.seller {
        assert_trade_state_change_is_valid(
            trade.state.clone(),
            TradeState::FiatDeposited,
            TradeState::EscrowReleased,
        )
        .unwrap();
    }

    // The arbitrator can only release the escrow if the trade.state is EscrowDisputed
    let arbitration_mode =
        (info.sender.clone() == arbitrator) & (trade.state == TradeState::EscrowDisputed);
    // let arbitration_mode = false;

    // If the sender is not the seller
    // and the sender is not the arbitrator while the trade.state is EscrowDisputed
    // throw Unauthorized
    if !(info.sender == trade.seller) & !arbitration_mode {
        return Err(TradeError::Unauthorized {
            owner: trade.seller,
            arbitrator: arbitrator,
            caller: info.sender,
        });
    }

    // TODO test funding timeout case
    // throws error if state is expired BUT arbitrator can release expired trades that have trade.state EscrowDisputed
    // If the escrow funding has expired, the Seller can only refund the escrow to himself, not release it to the Buyer
    if (env.block.time.seconds() > trade.created_at + FUNDING_TIMEOUT) & !arbitration_mode {
        // TODO handle different expiration options
        return Err(TradeError::Expired {
            timeout: FUNDING_TIMEOUT,
            expired_at: trade.created_at + FUNDING_TIMEOUT,
            created_at: trade.created_at,
        });
    }

    //Load and check balance
    let balance_result = deps.querier.query_balance(&env.contract.address, "uusd");
    if balance_result.is_err() {
        return Err(TradeError::ReleaseError {
            message: "Contract has no funds.".to_string(),
        });
    }

    let offer = get_offer(&deps.as_ref(), &trade);

    //Update trade State to TradeState::EscrowReleased or TradeState::SettledFor(Maker|Taker)
    let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

    if !arbitration_mode {
        trade.state = TradeState::EscrowReleased;
    } else if (offer.offer_type == OfferType::Buy) & (offer.owner == trade.buyer) {
        trade.state = TradeState::SettledForMaker;
    } else {
        trade.state = TradeState::SettledForTaker;
    }

    state_storage(deps.storage).save(&trade).unwrap();

    //Calculate fees and final release amount
    let mut send_msgs: Vec<SubMsg> = Vec::new();

    let factory_cfg: FactoryConfig =
        get_factory_config(&deps.querier, trade.factory_addr.to_string());

    //Collect Fee
    // let local_terra_fee = Coin::new(localterra_fee(trade.ust_amount.clone()).u128(), "uusd");
    // let fee_collector = factory_cfg.fee_collector_addr.clone();
    // send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
    // to_address: fee_collector.into_string(),
    // amount: vec![local_terra_fee],
    // })));

    let ust = Coin::new(trade.ust_amount.u128(), "uusd");
    //Release amount
    let release_amount = if offer.offer_type == OfferType::Buy {
        // //TODO: Move to a method
        // let ltfee = localterra_fee(trade.ust_amount);
        // let ltfee_coin = Coin::new(ltfee.u128(), "uusd");
        // let ltfee_tax = compute_tax(&deps.querier, &ltfee_coin).unwrap();

        // let mut arbitration_fee_inc_tax = Uint128::zero();
        // if arbitration_mode {
        //     // Pay arbitration fee
        //     let arbitration_rate = 10u128; // TODO move fee to constant
        //     let arbitration_coin = Coin::new(
        //         trade
        //             .ust_amount
        //             .u128()
        //             .clone()
        //             .checked_div(arbitration_rate)
        //             .unwrap(),
        //         "uusd",
        //     );

        //     arbitration_fee_inc_tax =
        //         arbitration_coin.amount + compute_tax(&deps.querier, &arbitration_coin).unwrap();

        //     send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        //         to_address: trade.arbitrator.clone().unwrap().to_string(),
        //         amount: vec![arbitration_coin],
        //     })));
        // }

        // let release_amount = trade
        //     .ust_amount
        //     .sub(ltfee)
        //     .sub(ltfee_tax)
        //     .sub(arbitration_fee_inc_tax);

        // let release_tax =
        //     compute_tax(&deps.querier, &Coin::new(release_amount.u128(), "uusd")).unwrap();

        // let deduction = ltfee.add(&ltfee_tax).add(&release_tax);

        // Coin::new(trade.ust_amount.sub(deduction).u128(), "uusd")

        ust
    } else {
        ust
    };

    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: trade.buyer.into_string(),
        amount: vec![release_amount],
    })));

    //Create Trade Registration message to be sent to the Trading Incentives contract.
    let register_trade_msg = SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: factory_cfg.trading_incentives_addr.to_string(),
        msg: to_binary(&TradingIncentivesMsg::RegisterTrade {
            trade: env.contract.address.to_string(),
            maker: offer.owner.to_string(),
        })
        .unwrap(),
        funds: vec![],
    }));
    send_msgs.push(register_trade_msg);

    let res = Response::new().add_submessages(send_msgs);
    Ok(res)
}

fn refund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade: TradeData,
) -> Result<Response, TradeError> {
    // Refund can only happen if:
    // 1) By anyone: TradeState::EscrowFunded and FundingTimeout is expired
    // 2) By assigned arbitrator: TradeState::EscrowDisputed

    let arbitration_mode = (info.sender == trade.arbitrator.clone().unwrap())
        & (trade.state == TradeState::EscrowDisputed);

    // anyone can try to refund, as long as the contract is expired
    // noone except arbitrator can refund if the trade is in arbitration
    let expired = env.block.time.seconds() > trade.created_at + FUNDING_TIMEOUT; // TODO test expiration case

    // TODO move to guard
    if !(expired || arbitration_mode)
    // If either is true, skip guard
    // TODO test this case
    {
        if !expired {
            return Err(TradeError::RefundErrorNotExpired {
                message:
                    "Only expired trades that are not disputed can be refunded by non-arbitrators."
                        .to_string(),
                trade: trade.state.to_string(),
            });
        }

        if !arbitration_mode {
            return Err(TradeError::RefundErrorNoArbitrationAllowed {
                message:
                    "Only expired trades that are not disputed can be refunded by non-arbitrators."
                        .to_string(),
                trade: trade.state.to_string(),
            });
        }
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    return if balance_result.is_ok() {
        let offer = get_offer(&deps.as_ref(), &trade);

        //Update TradeData to TradeState::Released or TradeState::SettledFor(Maker|Taker)
        let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

        if !arbitration_mode {
            trade.state = TradeState::EscrowRefunded;
        } else if (offer.offer_type == OfferType::Buy) & (offer.owner == trade.buyer) {
            trade.state = TradeState::SettledForTaker;
        } else {
            trade.state = TradeState::SettledForMaker;
        }

        state_storage(deps.storage).save(&trade).unwrap();

        // Pay arbitration fee
        if arbitration_mode {
            let mut balance = balance_result.unwrap();

            let fee_rate: Uint128 = Uint128::new(10);
            let fee_amount = balance[0].amount.multiply_ratio(Uint128::new(1), fee_rate); // TODO support multiple coins
            let mut fee = balance.clone();
            fee[0].amount = fee_amount;
            balance[0].amount = balance[0].amount - fee_amount;

            let seller_msg = create_send_msg(&deps, trade.seller, balance);

            let arbitrator_msg = create_send_msg(&deps, trade.arbitrator.clone().unwrap(), fee);

            let res = Response::new()
                .add_submessage(SubMsg::new(seller_msg))
                .add_submessage(SubMsg::new(arbitrator_msg));
            Ok(res)
        } else {
            let balance = balance_result.unwrap();
            let send_msg = create_send_msg(&deps, trade.seller, balance);
            let res = Response::new().add_submessage(SubMsg::new(send_msg));

            Ok(res)
        }
    } else {
        Err(TradeError::RefundErrorNoFunds {
            message: "Contract has no funds.".to_string(),
            trade: trade.state.to_string(),
        })
    };
}

fn get_ust_amount(info: MessageInfo) -> Uint128 {
    let ust = &info.funds.iter().find(|c| c.denom.eq("uusd"));
    return match ust {
        None => Uint128::zero(),
        Some(c) => c.amount,
    };
}

pub fn localterra_fee(amount: Uint128) -> Uint128 {
    amount.clone().checked_div(Uint128::new(100u128)).unwrap()
}

fn create_send_msg(deps: &DepsMut, to_address: Addr, coins: Vec<Coin>) -> CosmosMsg {
    let mut coins_without_tax: Vec<Coin> = Vec::new();
    coins
        .iter()
        .for_each(|c| coins_without_tax.push(deduct_tax(&deps.querier, c.clone()).unwrap()));
    CosmosMsg::Bank(BankMsg::Send {
        to_address: to_address.to_string(),
        amount: coins_without_tax,
    })
}
