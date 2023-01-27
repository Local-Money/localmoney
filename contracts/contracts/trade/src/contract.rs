use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, CustomQuery, Deps,
    DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult, SubMsg, Uint128, Uint256,
    WasmMsg,
};
use cw2::{get_contract_version, set_contract_version};
use std::ops::{Mul, Sub};

use cw20::Denom;
use localmoney_protocol::currencies::FiatCurrency;
use localmoney_protocol::denom_utils::denom_to_string;
use localmoney_protocol::errors::ContractError;
use localmoney_protocol::errors::ContractError::{
    FundEscrowError, HubAlreadyRegistered, InvalidDenom, InvalidParameter, InvalidTradeState,
    OfferNotFound, RefundErrorNotExpired, TradeExpired,
};
use localmoney_protocol::guards::{
    assert_migration_parameters, assert_ownership, assert_sender_is_buyer_or_seller,
    assert_trade_state_and_type, assert_trade_state_change, assert_trade_state_change_is_valid,
    assert_value_in_range, validate_min_max_items_per_page,
};
use localmoney_protocol::hub::HubConfig;
use localmoney_protocol::hub_utils::{get_hub_admin, get_hub_config, register_hub_internal};
use localmoney_protocol::offer::{load_offer, Arbitrator, OfferType, TradeInfo};
use localmoney_protocol::price::{query_fiat_price_for_denom, DenomFiatPrice};
use localmoney_protocol::profile::{
    load_profile, update_profile_contact_msg, update_profile_trades_count_msg,
};
use localmoney_protocol::trade::{
    arbitrators, calc_denom_fiat_price, ArbitratorModel, ConversionRoute, ConversionStep,
    ExecuteMsg, FeeInfo, InstantiateMsg, MigrateMsg, NewTrade, QueryMsg, Swap, SwapMsg, Trade,
    TradeModel, TradeResponse, TradeState, TradeStateItem, TraderRole, DENOM_CONVERSION_ROUTE,
    DENOM_CONVERSION_STEP,
};
pub const SWAP_REPLY_ID: u64 = 1u64;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();

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
        ExecuteMsg::Create(new_trade) => create_trade(deps, env, info, new_trade),
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
        ExecuteMsg::RegisterConversionRouteForDenom { denom, route } => {
            register_conversion_route_for_denom(deps, info, denom, route)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let previous_contract_version = get_contract_version(deps.storage).unwrap();

    assert_migration_parameters(
        previous_contract_version.clone(),
        CONTRACT_NAME.to_string(),
        CONTRACT_VERSION,
    )
    .unwrap();

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    // If the structure of the data in storage changes, we must treat it here

    Ok(Response::default()
        .add_attribute("previous_version", previous_contract_version.version)
        .add_attribute("new_version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}

fn create_trade(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_trade: NewTrade,
) -> Result<Response, ContractError> {
    // Load Hub Cfg
    let hub_cfg = get_hub_config(deps.as_ref());

    // Load Offer
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
    assert_value_in_range(offer.min_amount, offer.max_amount, new_trade.amount.clone())?;

    // Can't create Trade with the same wallet
    if info.sender.eq(&offer.owner) {
        return Err(ContractError::Unauthorized {
            owner: offer.owner,
            caller: info.sender,
        });
    }

    // Check if new_trade.amount in fiat is lower than the trade limit at hub_cfg
    let offer_denom_usd_price = query_fiat_price_for_denom(
        &deps.querier,
        offer.denom.clone(),
        FiatCurrency::USD,
        hub_cfg.price_addr.to_string(),
    )
    .unwrap_or(DenomFiatPrice {
        denom: offer.denom.clone(),
        fiat: FiatCurrency::USD,
        price: Uint256::from_u128(0),
    });
    let offer_usd_price = calc_denom_fiat_price(offer.rate, offer_denom_usd_price.price);
    let new_trade_amount = Uint256::from_u128(new_trade.amount.u128());
    let usd_trade_amount = (new_trade_amount * offer_usd_price)
        .checked_div(Uint256::from_u128(100u128))
        .unwrap_or(Uint256::zero());
    let usd_trade_amount = usd_trade_amount
        .checked_div(Uint256::from_u128(1_000_000u128))
        .unwrap_or(Uint256::zero());

    // The min amount
    let min_amount = Uint256::from_u128(hub_cfg.trade_limit_min);
    let max_amount = Uint256::from_u128(hub_cfg.trade_limit_max);

    // Check that usd_trade_amount is lower or equal than the trade limit and return error if not.
    if usd_trade_amount < min_amount || usd_trade_amount > max_amount {
        return Err(ContractError::InvalidTradeAmount {
            amount: usd_trade_amount,
            min_amount,
            max_amount,
        });
    }

    //Freeze the Denom price in Fiat using the rate set on Offer by the Maker
    let denom_fiat_price = query_fiat_price_for_denom(
        &deps.querier,
        offer.denom.clone(),
        offer.fiat_currency.clone(),
        hub_cfg.price_addr.to_string(),
    )
    .unwrap_or(DenomFiatPrice {
        denom: offer.denom.clone(),
        fiat: offer.fiat_currency.clone(),
        price: Uint256::from_u128(0),
    });
    let denom_final_price = calc_denom_fiat_price(offer.rate, denom_fiat_price.price);
    if denom_final_price.is_zero() {
        return Err(ContractError::InvalidPriceForDenom {});
    }

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

    let trades_count = TradeModel::size(deps.storage) as u64;
    let trade_id = trades_count + 1;

    let new_trade_state = TradeStateItem {
        actor: new_trade.taker.clone(),
        state: TradeState::RequestCreated,
        timestamp: env.block.time.seconds(),
    };
    let trade_state_history = vec![new_trade_state];

    let mut sub_msgs = vec![];
    sub_msgs.push(update_profile_contact_msg(
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
            buyer.clone(),
            seller.clone(),
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
            denom_final_price,
            trade_state_history,
        ),
    )
    .trade;

    let mut profile_submsgs = create_update_trades_count_msgs(
        hub_cfg.profile_addr.to_string(),
        trade.buyer.clone(),
        trade.seller.clone(),
        TradeState::RequestCreated,
    );
    sub_msgs.append(&mut profile_submsgs);

    let denom_str = denom_to_string(&trade.denom);
    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "create_trade")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("offer_id", offer.id.to_string())
        .add_attribute("owner", offer.owner.to_string())
        .add_attribute("amount", trade.amount.to_string())
        .add_attribute("denom", denom_str)
        .add_attribute("denom_fiat_price", denom_fiat_price.price.to_string())
        .add_attribute("offer_rate", offer.rate.to_string())
        .add_attribute("taker", new_trade.taker.to_string())
        .add_attribute("usd_trade_amount", usd_trade_amount.to_string())
        .add_attribute("offer_usd_price", offer_usd_price.to_string());

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Trade { id } => to_binary(&query_trade(env, deps, id)?),
        QueryMsg::Trades {
            user,
            role,
            limit,
            last,
        } => to_binary(&query_trades(env, deps, user, role, limit, last)?),
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

fn register_hub<T: CustomQuery>(
    deps: DepsMut<T>,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_trade<T: CustomQuery>(env: Env, deps: Deps<T>, id: u64) -> StdResult<TradeInfo> {
    let hub_config = get_hub_config(deps);
    let state = TradeModel::from_store(deps.storage, id);

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

pub fn query_trades<T: CustomQuery>(
    env: Env,
    deps: Deps<T>,
    user: Addr,
    role: TraderRole,
    limit: u32,
    last: Option<u64>,
) -> StdResult<Vec<TradeInfo>> {
    let mut trades_infos: Vec<TradeInfo> = vec![];
    let hub_config = get_hub_config(deps);
    let limit = validate_min_max_items_per_page(limit) as usize;

    let trade_results = match role {
        TraderRole::Arbitrator => {
            TradeModel::trades_by_arbitrator(deps.storage, user.to_string(), limit, last)
        }
        TraderRole::Trader => {
            TradeModel::trades_by_trader(deps.storage, user.to_string(), limit, last)
        }
    }
    .unwrap();

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
    trade_id: u64,
    maker_contact: Option<String>,
) -> Result<Response, ContractError> {
    // Load HubConfig, Trade & Offer
    let hub_config = get_hub_config(deps.as_ref());
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
    let offer = load_offer(
        &deps.querier.clone(),
        trade.offer_id.clone(),
        trade.offer_contract.to_string(),
    )
    .unwrap()
    .offer;

    // Ensure the message has the correct funds
    let trade_denom = &denom_to_string(&trade.denom);
    let balance = match info.funds.first().unwrap() {
        coin if coin.denom.eq(trade_denom) => coin.clone(),
        _ => {
            let received = info.funds.first().unwrap_or(&Coin::default()).denom.clone();
            return Err(InvalidDenom {
                expected: trade_denom.clone(),
                received,
            });
        }
    };
    let fee_info = calculate_fees(&hub_config, trade.amount.clone());

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
    assert_ownership(info.sender.clone(), trade.seller.clone())?;

    // If seller_contact is not already defined it needs to be defined here
    if trade.seller_contact.is_none() {
        if maker_contact.is_some() {
            // Set maker_contact as seller_contact
            trade.seller_contact = maker_contact
        } else {
            return Err(InvalidParameter {
                parameter: "maker_contact".to_string(),
                message: Some("At this point the maker_contact can not be undefined".to_string()),
            });
        }
    }

    // Ensure TradeState::Created for Sell and TradeState::Accepted for Buy orders
    assert_trade_state_and_type(&trade, &offer.offer_type)?;

    // If the sender funding the escrow is the maker, the fee must be added on top of the trade amount
    let total_fees = if offer.owner.eq(&info.sender) {
        fee_info.total_fees()
    } else {
        Uint128::new(0u128)
    };

    // Ensure the amount sent is equal to the trade amount + fees
    if balance.amount != trade.amount + total_fees {
        return Err(FundEscrowError {
            required_amount: trade.amount + total_fees,
            sent_amount: balance.amount,
        });
    }

    // Set the state to EscrowFunded and store the trade
    trade.set_state(TradeState::EscrowFunded, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    let mut sub_msgs: Vec<SubMsg> = vec![];

    if info.sender.eq(&offer.owner) {
        let mut profile_submsgs = create_update_trades_count_msgs(
            hub_config.profile_addr.to_string(),
            trade.buyer.clone(),
            trade.seller.clone(),
            TradeState::EscrowFunded,
        );
        sub_msgs.append(&mut profile_submsgs);
    }

    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "fund_escrow")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("trade.amount", trade.amount.clone().to_string())
        .add_attribute("sent_amount", balance.amount.to_string())
        .add_attribute("seller", info.sender)
        .add_attribute("state", trade.get_state().to_string());
    Ok(res)
}

// Only makers can use this action
fn accept_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
    maker_contact: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
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

    // Load Hub Cfg
    let hub_config = get_hub_config(deps.as_ref());

    let sub_msgs = create_update_trades_count_msgs(
        hub_config.profile_addr.to_string(),
        trade.buyer.clone(),
        trade.seller.clone(),
        TradeState::RequestAccepted,
    );

    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "accept_request")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("state", trade.get_state().to_string());

    Ok(res)
}

fn fiat_deposited(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
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
    // Sets the time that will enable the dispute
    let enables_dispute_at = env.block.time.seconds() + hub_config.trade_dispute_timer;
    trade.enables_dispute_at = Some(enables_dispute_at);

    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "fiat_deposited")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("state", trade.get_state().to_string());

    Ok(res)
}

fn cancel_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
    // Only the buyer or seller can cancel the trade.
    assert_sender_is_buyer_or_seller(
        info.sender.clone(),
        trade.buyer.clone(),
        trade.seller.clone(),
    )
    .unwrap();

    // The trade can be canceled if the state is RequestAccepted or RequestCreated
    let mut allowed_states = vec![TradeState::RequestAccepted, TradeState::RequestCreated];

    // Only the buyer can cancel the trade if it is already funded
    if info.sender.clone().eq(&trade.buyer.clone()) {
        allowed_states.push(TradeState::EscrowFunded)
    }

    assert_trade_state_change(
        trade.get_state(),
        allowed_states,
        TradeState::RequestCanceled,
    )
    .unwrap();

    let mut sub_msgs: Vec<SubMsg> = vec![];
    // Should not be called when the current state is TradeState::RequestCreated
    if vec![TradeState::EscrowFunded, TradeState::RequestAccepted].contains(&trade.get_state()) {
        // Load hub config
        let hub_config = get_hub_config(deps.as_ref());
        let mut profile_submsgs = create_update_trades_count_msgs(
            hub_config.profile_addr.to_string(),
            trade.buyer.clone(),
            trade.seller.clone(),
            TradeState::EscrowCanceled,
        );
        sub_msgs.append(&mut profile_submsgs)
    }

    if trade.get_state().eq(&TradeState::EscrowFunded) {
        // Update trade State to TradeState::EscrowCanceled
        trade.set_state(TradeState::EscrowCanceled, &env, &info);
    } else {
        // Update trade State to TradeState::RequestCanceled
        trade.set_state(TradeState::RequestCanceled, &env, &info);
    }
    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "cancel_request")
        .add_submessages(sub_msgs);
    Ok(res)
}

fn release_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    // Load trade and validate that permission and state are valid.
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
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
    let hub_config = get_hub_config(deps.as_ref());

    // Update trade State to TradeState::EscrowReleased
    trade.set_state(TradeState::EscrowReleased, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    // Load the offer related to this trade
    let offer_response = load_offer(
        &deps.querier,
        trade.offer_id.clone(),
        hub_config.offer_addr.to_string(),
    )
    .unwrap();

    let mut send_msgs: Vec<SubMsg> = Vec::new();
    // Calculate and add protocol fees
    let mut release_amount = trade.amount.clone();
    let fee_info = add_protocol_fees_msgs(
        deps,
        &mut send_msgs,
        &release_amount,
        trade_denom.clone(),
        &hub_config,
    );

    // Only deducts fees from the release_amount if the maker (offer owner) is the buyer
    if trade.buyer.eq(&offer_response.offer.owner) {
        release_amount = release_amount.sub(fee_info.total_fees());
    }

    let mut profile_submsgs = create_update_trades_count_msgs(
        hub_config.profile_addr.to_string(),
        trade.buyer.clone(),
        trade.seller.clone(),
        TradeState::EscrowReleased,
    );
    send_msgs.append(&mut profile_submsgs);

    // Send tokens to buyer
    send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: trade.buyer.to_string(),
        amount: vec![Coin::new(release_amount.u128(), trade_denom.clone())],
    })));

    let res = Response::new()
        .add_submessages(send_msgs)
        .add_attribute("action", "release_escrow")
        .add_attribute("trade_id", trade_id.to_string())
        .add_attribute("state", trade.get_state().clone().to_string())
        .add_attribute("trade_denom", denom_to_string(&trade.denom))
        .add_attribute("total_amount", trade.amount.u128().to_string());
    Ok(res)
}

fn refund_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
) -> Result<Response, ContractError> {
    // Refund can only happen if trade state is TradeState::EscrowFunded and FundingTimeout is expired
    let trade = TradeModel::from_store(deps.storage, trade_id);

    //
    assert_trade_state_change(
        trade.get_state(),
        vec![TradeState::EscrowFunded, TradeState::EscrowCanceled],
        TradeState::EscrowFunded,
    )
    .unwrap();

    // anyone can try to refund, as long as the trade is funded and expired or escrow canceled
    let block_time = env.block.time.seconds();
    if trade.get_state().eq(&TradeState::EscrowFunded) && !trade.request_expired(block_time) {
        return Err(RefundErrorNotExpired {
            message:
                "Only expired trades that are not disputed can be refunded by non-arbitrators."
                    .to_string(),
            trade: trade.get_state().to_string(),
        });
    }

    let mut trade: Trade = TradeModel::from_store(deps.storage, trade_id);

    //Update trade state to TradeState::EscrowRefunded
    trade.set_state(TradeState::EscrowRefunded, &env, &info);
    TradeModel::store(deps.storage, &trade).unwrap();

    let hub_config = get_hub_config(deps.as_ref());

    let mut sub_msgs: Vec<SubMsg> = create_update_trades_count_msgs(
        hub_config.profile_addr.to_string(),
        trade.buyer.clone(),
        trade.seller.clone(),
        TradeState::EscrowRefunded,
    );

    let amount = trade.amount.clone();
    let denom = denom_to_string(&trade.denom);
    let refund_amount = vec![Coin::new(amount.u128(), denom.clone())];
    sub_msgs.push(SubMsg::new(create_send_msg(trade.seller, refund_amount)));
    let res = Response::new()
        .add_attribute("action", "refund_escrow")
        .add_submessages(sub_msgs);
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

    let create_profile_sub_msg = update_profile_contact_msg(
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
    trade_id: u64,
    buyer_contact: String,
    seller_contact: String,
) -> Result<Response, ContractError> {
    let mut trade = TradeModel::from_store(deps.storage, trade_id);
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

    // The `enables_dispute_at` is defined in the fiat_deposited
    let enables_dispute_at = trade.enables_dispute_at.unwrap();
    let current_block_time = env.block.time.seconds();
    // Returns an error if is too early to open a dispute
    if enables_dispute_at > current_block_time {
        let time_to_dispute = enables_dispute_at - current_block_time;
        return Err(ContractError::PrematureDisputeRequest { time_to_dispute });
    }

    // Update trade State to TradeState::Disputed and sets arbitrator
    trade.set_state(TradeState::EscrowDisputed, &env, &info);
    trade.arbitrator_buyer_contact = Some(buyer_contact);
    trade.arbitrator_seller_contact = Some(seller_contact);
    TradeModel::store(deps.storage, &trade).unwrap();

    let res = Response::new()
        .add_attribute("action", "dispute_escrow")
        .add_attribute("trade_id", trade.id.to_string())
        .add_attribute("state", trade.get_state().to_string())
        .add_attribute("arbitrator", trade.arbitrator.to_string());

    Ok(res)
}

fn settle_dispute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    trade_id: u64,
    winner: Addr,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());
    let mut trade = TradeModel::from_store(deps.storage, trade_id);

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

    // Collect Protocol Fees
    let trade_denom = denom_to_string(&trade.denom);
    let mut send_msgs: Vec<SubMsg> = vec![];
    let fee_info = add_protocol_fees_msgs(
        deps,
        &mut send_msgs,
        &trade.amount,
        trade_denom.clone(),
        &hub_config,
    );

    // Pay arbitration fee
    let arbitration_fee_amount = trade.amount.mul(hub_config.arbitration_fee_pct);
    let mut release_amount = trade.amount.sub(arbitration_fee_amount);

    // Only deducts fees from the release_amount if the maker (offer owner) is the buyer
    if trade.buyer.eq(&offer.owner) {
        release_amount = release_amount.sub(fee_info.total_fees());
    }

    // Send funds to winner and arbitrator
    let denom = denom_to_string(&trade.denom);
    let arbitration_fee = vec![Coin::new(arbitration_fee_amount.u128(), denom.clone())];
    let winner_amount = vec![Coin::new(release_amount.u128(), denom.clone())];
    send_msgs.push(SubMsg::new(create_send_msg(winner.clone(), winner_amount)));
    send_msgs.push(SubMsg::new(create_send_msg(
        trade.arbitrator.clone(),
        arbitration_fee,
    )));

    // Create Update Profile SubMsgs
    let profile_submsgs = create_update_trades_count_msgs(
        hub_config.profile_addr.to_string(),
        trade.buyer.clone(),
        trade.seller.clone(),
        trade.get_state(),
    );

    let res = Response::new()
        .add_attribute("arbitrator", trade.arbitrator.to_string())
        .add_attribute("winner", winner.to_string())
        .add_attribute("maker", maker.to_string())
        .add_attribute("taker", taker.to_string())
        .add_submessages(profile_submsgs)
        .add_submessages(send_msgs);
    Ok(res)
}

/// Registers a conversion route for a given denom.
fn register_conversion_route_for_denom(
    deps: DepsMut,
    info: MessageInfo,
    denom: Denom,
    route: Vec<ConversionRoute>,
) -> Result<Response, ContractError> {
    // Check if caller is the hub contract's admin
    let admin = get_hub_admin(deps.as_ref()).addr;
    assert_ownership(info.sender, admin)?;

    // Store conversion route
    let denom = denom_to_string(&denom);
    DENOM_CONVERSION_ROUTE
        .save(deps.storage, &denom, &route)
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "register_conversion_route_for_denom")
        .add_attribute("denom", denom);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        SWAP_REPLY_ID => handle_swap_reply(deps, msg),
        id => Err(ContractError::UnknownReplyId { reply_id: id }),
    }
}

/// Handle the reply from the swap contract.
/// It checkes if the received asset is LOCAL, if it is, it burns it.
/// Otherwise, it continues the conversion following the ConversionRoute.
fn handle_swap_reply(deps: DepsMut, _msg: Reply) -> Result<Response, ContractError> {
    // Load Hub Config
    let hub_config = get_hub_config(deps.as_ref());
    let contract_address = &hub_config.trade_addr.to_string();

    // Load the ConversionRoute for the current step denom's.
    let conversion_step = DENOM_CONVERSION_STEP.load(deps.storage).unwrap();
    let next_step = (conversion_step.step + 1) as usize;
    let trade_denom = denom_to_string(&conversion_step.trade_denom);
    let conversion_route = DENOM_CONVERSION_ROUTE
        .load(deps.storage, &trade_denom)
        .unwrap();

    // Query the contract's balance of the ask_asset of the current step of the conversion route.
    let received_denom =
        denom_to_string(&conversion_route[conversion_step.step as usize].ask_asset);
    let mut received_asset_balance = deps
        .querier
        .query_balance(contract_address, received_denom.clone())
        .unwrap();
    // !!! Ensure that we're using the balance difference for the execution instead of the whole balance of the contract.
    received_asset_balance.amount = received_asset_balance
        .amount
        .sub(conversion_step.step_previous_balance.amount);

    // Check that the different between the current ask_asset_balance and the previous ask_asset_balance is greater than zero.
    if received_asset_balance.amount.is_zero() {
        return Err(ContractError::SwapErrorInvalidAmount {});
    }

    let conversion_step_attr = (
        "conversion_step",
        format!(
            "{} out of {}",
            (conversion_step.step + 1),
            conversion_route.len()
        ),
    );
    let event_attr = ("event", "swap_reply".to_string());

    // Check if the received asset denom is the LOCAL Denom. If so, we can burn the asset and return.
    // If not, we need to swap the asset for the next denom in the conversion route.
    let local_denom = denom_to_string(&hub_config.local_denom);
    return if received_asset_balance.denom.eq(&local_denom) {
        // Burn $LOCAL
        let burn_msg = CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![received_asset_balance.clone()],
        });

        // Reset the DENOM_CONVERSION_STEP
        DENOM_CONVERSION_STEP.remove(deps.storage);

        let res = Response::new()
            .add_attributes(vec![
                event_attr,
                conversion_step_attr,
                ("burn_amount", received_asset_balance.amount.to_string()),
                ("received_denom", local_denom),
            ])
            .add_submessage(SubMsg::new(burn_msg));
        Ok(res)
    } else if conversion_route.len() > next_step {
        // Load next step in the conversion route.
        let route_step = conversion_route.get(next_step).unwrap();
        // Get the current balance of route_step.ask_asset.
        let route_step_asset_balance = deps
            .querier
            .query_balance(contract_address, denom_to_string(&route_step.ask_asset))
            .unwrap();

        // Update the DENOM_CONVERSION_STEP
        DENOM_CONVERSION_STEP
            .save(
                deps.storage,
                &ConversionStep {
                    trade_denom: conversion_step.trade_denom.clone(),
                    step_previous_balance: route_step_asset_balance,
                    step: conversion_step.step + 1,
                },
            )
            .unwrap();

        // Return a Swap SubMsg to swap the received asset for the next denom in the conversion route.
        let res = Response::new()
            .add_attributes(vec![
                event_attr,
                conversion_step_attr,
                ("swap_amount", received_asset_balance.amount.to_string()),
                ("received_denom", received_asset_balance.denom.clone()),
            ])
            .add_submessage(SubMsg {
                id: SWAP_REPLY_ID,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: route_step.pool.to_string(),
                    msg: to_binary(&SwapMsg { swap: Swap {} }).unwrap(),
                    funds: vec![received_asset_balance],
                }),
                gas_limit: None,
                reply_on: ReplyOn::Success,
            });
        Ok(res)
    } else {
        Err(ContractError::SwapErrorMissingDenom {
            expected_denom: received_denom,
        })
    };
}

// Create sub messages for updating trades count fields on maker and taker profiles
fn create_update_trades_count_msgs(
    profile_addr: String,
    buyer: Addr,
    seller: Addr,
    trade_state: TradeState,
) -> Vec<SubMsg> {
    let update_buyer =
        update_profile_trades_count_msg(profile_addr.clone(), buyer, trade_state.clone());
    let update_seller =
        update_profile_trades_count_msg(profile_addr.clone(), seller, trade_state.clone());
    vec![update_buyer, update_seller]
}

// region utils
// Creates a BankMsg::Send message
fn create_send_msg(to_address: Addr, amount: Vec<Coin>) -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: to_address.to_string(),
        amount,
    })
}

/// Returns a FeeInfo struct containing the calculated fees and the final release amount.
fn calculate_fees(hub_config: &HubConfig, amount: Uint128) -> FeeInfo {
    let burn_amount = amount.mul(hub_config.burn_fee_pct);
    let chain_amount = amount.mul(hub_config.chain_fee_pct);
    let warchest_amount = amount.mul(hub_config.warchest_fee_pct);

    FeeInfo {
        burn_amount,
        chain_amount,
        warchest_amount,
    }
}

// Adds protocol fees to the given send_msgs.
fn add_protocol_fees_msgs(
    deps: DepsMut,
    send_msgs: &mut Vec<SubMsg>,
    release_amount: &Uint128,
    trade_denom: String,
    hub_cfg: &HubConfig,
) -> FeeInfo {
    // Calculate fees
    let fee_info = calculate_fees(hub_cfg, release_amount.clone());

    // Protocol Fee (Burn)
    if !fee_info.burn_amount.is_zero() {
        //If coin being traded is not $LOCAL, swap it and burn it on swap reply.
        let local_denom = denom_to_string(&hub_cfg.local_denom);
        if trade_denom.ne(&local_denom) {
            // Load the ConversionRoute route for trade_denom
            let conversion_route = DENOM_CONVERSION_ROUTE
                .load(deps.storage, &trade_denom)
                .unwrap()
                .first()
                .unwrap()
                .clone();

            // Query the contract's balance of the the ConversionRoute's ask_asset
            let ask_asset_balance = deps
                .querier
                .query_balance(
                    hub_cfg.trade_addr.to_string(),
                    denom_to_string(&conversion_route.ask_asset),
                )
                .unwrap_or(Coin::new(
                    0u128,
                    denom_to_string(&conversion_route.ask_asset),
                ));

            // Store the ConversionStep
            DENOM_CONVERSION_STEP
                .save(
                    deps.storage,
                    &ConversionStep {
                        trade_denom: Denom::Native(trade_denom.clone()),
                        step_previous_balance: ask_asset_balance,
                        step: 0,
                    },
                )
                .unwrap();

            // Add message to swap the burn_amount and burn it on swap reply
            send_msgs.push(SubMsg {
                id: SWAP_REPLY_ID,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: conversion_route.pool.to_string(),
                    msg: to_binary(&SwapMsg { swap: Swap {} }).unwrap(),
                    funds: vec![coin(fee_info.burn_amount.u128(), trade_denom.clone())],
                }),
                gas_limit: None,
                reply_on: ReplyOn::Success,
            });
        } else {
            //If coin being traded is $LOCAL, add message burning the local_burn amount
            send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![coin(fee_info.burn_amount.u128(), local_denom.clone())],
            })));
        }
    }

    // Chain Fee Sharing
    if !fee_info.chain_amount.is_zero() {
        send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: hub_cfg.chain_fee_collector_addr.to_string(),
            amount: vec![coin(fee_info.chain_amount.u128(), trade_denom.clone())],
        })));
    }

    // Warchest
    if !fee_info.warchest_amount.is_zero() {
        send_msgs.push(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: hub_cfg.warchest_addr.to_string(),
            amount: vec![coin(fee_info.warchest_amount.u128(), trade_denom.clone())],
        })));
    }
    fee_info
}
//endregion
