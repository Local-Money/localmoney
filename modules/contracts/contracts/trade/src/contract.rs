use std::ops::{Add, Sub};
use std::str::FromStr;

use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, QueryRequest, Response, StdResult, SubMsg, Uint128, WasmMsg,
    WasmQuery,
};

use localterra_protocol::factory::Config as FactoryConfig;
use localterra_protocol::factory_util::get_factory_config;
use localterra_protocol::offer::{
    Arbitrator, Config as OfferConfig, Offer, OfferType, QueryMsg as OfferQueryMsg,
};
use localterra_protocol::trade::{ExecuteMsg, InstantiateMsg, QueryMsg, TradeData, TradeState};
use localterra_protocol::trading_incentives::ExecuteMsg as TradingIncentivesMsg;

use crate::errors::TradeError;
use crate::state::{state as state_storage, state_read};
use crate::taxation::{compute_tax, deduct_tax};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
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

    //TODO: it's probably a good idea to store this kind of configuration in a Gov contract.
    let expire_height = env.block.height + 600; //Roughly 1h.

    //Check that ust_amount is inside Offer limits
    let amount = Uint128::new(u128::from_str(msg.ust_amount.as_str()).unwrap());
    if amount > offer.max_amount || amount < offer.min_amount {
        return Err(TradeError::AmountError {
            amount,
            min_amount: offer.min_amount,
            max_amount: offer.max_amount,
        });
    }

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
    let mut trade = TradeData {
        addr: env.contract.address.clone(),
        factory_addr: offers_cfg.factory_addr.clone(),
        buyer: buyer,   // buyer
        seller: seller, // seller
        offer_contract: offer_contract.clone(),
        offer_id,
        taker_contact: msg.taker_contact,
        arbitrator: None,
        state: TradeState::Created,
        expire_height,
        ust_amount: amount,
        asset: offer.fiat_currency,
    };

    //Set state to EscrowFunded if enough UST was sent in the message.
    if !info.funds.is_empty() {
        //TODO: Check for Luna or other Terra native tokens.
        let ust_amount = get_ust_amount(info.clone());
        if ust_amount >= amount {
            trade.state = TradeState::EscrowFunded
        }
    }

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
        ExecuteMsg::Refund {} => refund(deps, env, info, state),
        ExecuteMsg::Release {} => release(deps, env, info, state),
        ExecuteMsg::Dispute {} => dispute(deps, env, info, state),
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
    //Check if trade is expired.
    if env.block.height >= trade.expire_height {
        return Err(TradeError::Expired {
            current_height: env.block.height,
            expire_height: trade.expire_height,
        });
    }
    // Check if escrow has already been funded
    // TODO also base this on actual balance, switch to cancelled state and refund automatically on diffs
    if trade.state == TradeState::EscrowFunded {
        return Err(TradeError::AlreadyFundedError {});
    }
    //TODO: Convert to UST if trade is for any other stablecoin or Luna,
    // skip conversion entirely if fee was paid in $LOCAL.
    let ust_amount = if !info.funds.is_empty() {
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
    let ust = Coin::new(ust_amount.clone().u128(), "uusd");

    let offer = load_offer(
        deps.querier.clone(),
        trade.offer_id,
        trade.offer_contract.to_string(),
    )
    .unwrap(); //at this stage, offer is guaranteed to exists.

    let fund_escrow_amount: Uint128 = match offer.offer_type {
        // TODO review this and avoid over-funding by returning diff
        OfferType::Sell => {
            let ltfee = localterra_fee(trade.ust_amount);
            let ltfee_coin = Coin::new(ltfee.u128(), "uusd");
            let ltfee_tax = compute_tax(&deps.querier, &ltfee_coin).unwrap();
            let release_tax = compute_tax(&deps.querier, &ust).unwrap();
            trade
                .ust_amount
                .add(ltfee.add(&ltfee_tax).add(&release_tax))
        }
        OfferType::Buy => trade.ust_amount,
    };
    if ust_amount >= fund_escrow_amount {
        trade.state = TradeState::EscrowFunded;
    } else {
        return Err(TradeError::FundEscrowError {
            required_amount: fund_escrow_amount.clone(),
            sent_amount: ust_amount.clone(),
        });
    }

    state_storage(deps.storage).save(&trade).unwrap();
    let res = Response::new()
        .add_attribute("action", "fund_escrow")
        .add_attribute("fund_amount", fund_escrow_amount.to_string())
        .add_attribute("ust_amount", ust_amount.to_string())
        .add_attribute("seller", info.sender);

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

fn dispute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    state: TradeData,
) -> Result<Response, TradeError> {
    if (info.sender != state.seller) & (info.sender != state.buyer) {
        return Err(TradeError::UnauthorizedDispute {
            seller: state.seller,
            buyer: state.buyer,
            caller: info.sender,
        });
    }

    // Update trade State to TradeState::Disputed
    let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

    trade.state = TradeState::Disputed;

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

    let res = Response::new();
    Ok(res)
}
fn release(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade: TradeData,
) -> Result<Response, TradeError> {
    let arbitration_mode =
        (info.sender == trade.arbitrator.clone().unwrap()) & (trade.state == TradeState::Disputed);

    //Check if seller can release
    if (info.sender != trade.seller) & !arbitration_mode {
        return Err(TradeError::Unauthorized {
            owner: trade.seller,
            arbitrator: trade.arbitrator.clone().unwrap(),
            caller: info.sender,
        });
    }

    // throws error if state is expired BUT arbitrator can release expired trades
    if (env.block.height > trade.expire_height) & !arbitration_mode {
        return Err(TradeError::Expired {
            expire_height: trade.expire_height,
            current_height: env.block.height,
        });
    }

    //Load and check balance
    // let balance_result = deps.querier.query_all_balances(&env.contract.address);
    let balance_result = deps.querier.query_balance(&env.contract.address, "uusd");
    if balance_result.is_err() {
        return Err(TradeError::ReleaseError {
            message: "Contract has no funds.".to_string(),
        });
    }

    let offer = get_offer(&deps.as_ref(), &trade);

    //Update trade State to TradeState::Released or TradeState::SettledFor(Maker|Taker)
    let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

    if !arbitration_mode {
        trade.state = TradeState::Released;
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
    let local_terra_fee = Coin::new(localterra_fee(trade.ust_amount.clone()).u128(), "uusd");
    let fee_collector = factory_cfg.fee_collector_addr.clone();
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: fee_collector.into_string(),
        amount: vec![local_terra_fee],
    })));

    let ust = Coin::new(trade.ust_amount.u128(), "uusd");
    //Release amount
    let release_amount = if offer.offer_type == OfferType::Buy {
        //TODO: Move to a method
        let ltfee = localterra_fee(trade.ust_amount);
        let ltfee_coin = Coin::new(ltfee.u128(), "uusd");
        let ltfee_tax = compute_tax(&deps.querier, &ltfee_coin).unwrap();

        let mut arbitration_fee_inc_tax = Uint128::zero();
        if arbitration_mode {
            // Pay arbitration fee
            let arbitration_rate = 10u128; // TODO move fee to constant
            let arbitration_coin = Coin::new(
                trade
                    .ust_amount
                    .u128()
                    .clone()
                    .checked_div(arbitration_rate)
                    .unwrap(),
                "uusd",
            );

            arbitration_fee_inc_tax =
                arbitration_coin.amount + compute_tax(&deps.querier, &arbitration_coin).unwrap();

            send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: trade.arbitrator.clone().unwrap().to_string(),
                amount: vec![arbitration_coin],
            })));
        }

        let release_amount = trade
            .ust_amount
            .sub(ltfee)
            .sub(ltfee_tax)
            .sub(arbitration_fee_inc_tax);

        let release_tax =
            compute_tax(&deps.querier, &Coin::new(release_amount.u128(), "uusd")).unwrap();

        let deduction = ltfee.add(&ltfee_tax).add(&release_tax);

        Coin::new(trade.ust_amount.sub(deduction).u128(), "uusd")
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

fn refund(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade: TradeData,
) -> Result<Response, TradeError> {
    let arbitration_mode =
        (info.sender == trade.arbitrator.clone().unwrap()) & (trade.state == TradeState::Disputed);

    // anyone can try to refund, as long as the contract is expired
    // noone except arbitrator can refund if the trade is in arbitration
    if (trade.expire_height > env.block.height)
        & ((trade.state != TradeState::Disputed) & !arbitration_mode)
    {
        return Err(TradeError::RefundError {
            message:
                "Only expired trades that are not disputed can be refunded by non-arbitrators."
                    .to_string(),
            trade: trade.state.to_string(),
        });
    }

    let balance_result = deps.querier.query_all_balances(&env.contract.address);
    return if balance_result.is_ok() {
        let offer = get_offer(&deps.as_ref(), &trade);

        //Update TradeData to TradeState::Released or TradeState::SettledFor(Maker|Taker)
        let mut trade: TradeData = state_storage(deps.storage).load().unwrap();

        if !arbitration_mode {
            trade.state = TradeState::Refunded;
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
        Err(TradeError::RefundError {
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
