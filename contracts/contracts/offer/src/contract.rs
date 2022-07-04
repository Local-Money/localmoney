use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, ContractResult, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, QueryRequest, Reply, ReplyOn, Response, StdResult, Storage, SubMsg,
    SubMsgResponse, Uint128, WasmMsg, WasmQuery,
};
use cw20::Denom;
use cw_storage_plus::Bound;

use localterra_protocol::constants::REQUEST_TIMEOUT;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::guards::{assert_min_g_max, assert_ownership, assert_range_0_to_99};
use localterra_protocol::hub_util::{
    get_contract_address_from_reply, get_hub_config, register_hub_internal, HubAddr, HUB_ADDR,
};
use localterra_protocol::offer::{
    offers, Arbitrator, ExecuteMsg, InstantiateMsg, Offer, OfferModel, OfferMsg, OfferState,
    OfferUpdateMsg, OffersCount, QueryMsg, TradeAddr, TradeInfo, TradesIndex,
};
use localterra_protocol::trade::{
    ExecuteMsg::Create as CreateTradeMsg, NewTrade, QueryMsg as TradeQueryMsg, Trade,
};

use crate::state::{arbitrators, offers_count_read, offers_count_storage, trades};
use localterra_protocol::errors::GuardError;
use localterra_protocol::errors::GuardError::HubAlreadyRegistered;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, GuardError> {
    offers_count_storage(deps.storage).save(&OffersCount { count: 0 })?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, GuardError> {
    match msg {
        ExecuteMsg::Create { offer } => create_offer(deps, env, info, offer),
        ExecuteMsg::UpdateOffer { offer_update } => update_offer(deps, env, info, offer_update),
        ExecuteMsg::NewArbitrator { arbitrator, asset } => {
            create_arbitrator(deps, env, info, arbitrator, asset)
        }
        ExecuteMsg::DeleteArbitrator { arbitrator, asset } => {
            delete_arbitrator(deps, env, info, arbitrator, asset)
        }
        ExecuteMsg::UpdateTradeArbitrator { arbitrator } => {
            // TODO merge this call with the query random arbitrator call
            execute_update_trade_arbitrator(deps, env, info, arbitrator)
        }
        ExecuteMsg::UpdateLastTraded {} => execute_update_last_traded(deps, env, info),
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HubAddr {} => to_binary(&query_hub_addr(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offers { fiat_currency } => {
            to_binary(&OfferModel::query_all_offers(deps.storage, fiat_currency)?)
        }
        QueryMsg::OffersQuery {
            owner,
            min,
            max,
            limit,
            order,
        } => to_binary(&OfferModel::query(deps, owner, min, max, limit, order)?),
        QueryMsg::OffersByType {
            offer_type,
            last_value,
            limit,
        } => to_binary(&OfferModel::query_by_type(
            deps, offer_type, last_value, limit,
        )?),
        QueryMsg::OffersByFiat {
            fiat_currency,
            last_value,
            limit,
        } => to_binary(&OfferModel::query_by_fiat(
            deps,
            fiat_currency,
            last_value,
            limit,
        )?),
        QueryMsg::OffersByTypeFiat {
            offer_type,
            fiat_currency,
            min,
            max,
            limit,
            order,
        } => to_binary(&OfferModel::query_by_type_fiat(
            deps,
            fiat_currency,
            min,
            max,
            limit,
            order,
        )?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps.storage, id)?),
        QueryMsg::TradesQuery {
            user,
            state,
            index,
            last_value,
            limit,
        } => to_binary(&query_trades(
            env,
            deps,
            deps.api.addr_validate(user.as_str()).unwrap(),
            index,
            last_value,
            limit,
        )?),
        QueryMsg::Arbitrator { arbitrator } => to_binary(&query_arbitrator(deps, arbitrator)?),
        QueryMsg::Arbitrators { last_value, limit } => {
            to_binary(&query_arbitrators(deps, last_value, limit)?)
        }
        QueryMsg::ArbitratorAsset { asset } => to_binary(&query_arbitrator_asset(deps, asset)?),
        QueryMsg::ArbitratorRandom {
            random_value,
            asset,
        } => to_binary(&query_arbitrator_random(
            deps,
            random_value as usize,
            asset,
        )?),
    }
}

pub fn create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, GuardError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let mut offers_count = offers_count_storage(deps.storage).load().unwrap();
    offers_count.count += 1;
    let offer_id = [msg.rate.clone().to_string(), offers_count.count.to_string()].join("_");

    let offer = OfferModel::create(
        deps.storage,
        Offer {
            id: offer_id,
            owner: info.sender.clone(),
            offer_type: msg.offer_type,
            fiat_currency: msg.fiat_currency.clone(),
            rate: msg.rate,
            denom: msg.denom,
            min_amount: msg.min_amount,
            max_amount: msg.max_amount,
            state: OfferState::Active,
            timestamp: env.block.time.seconds(),
            last_traded_at: 0,
            trades_count: 0,
        },
    )
    .offer;

    offers_count_storage(deps.storage)
        .save(&offers_count)
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "create_offer")
        .add_attribute("type", offer.offer_type.to_string())
        .add_attribute("id", offer.id.to_string())
        .add_attribute("rate", offer.rate.to_string())
        .add_attribute("min_amount", offer.min_amount.to_string())
        .add_attribute("max_amount", offer.max_amount.to_string())
        .add_attribute("owner", offer.owner);

    Ok(res)
}

pub fn execute_update_trade_arbitrator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    arbitrator: Addr,
) -> Result<Response, GuardError> {
    // TODO assert the calling contract can only update its own arbitrator and only if the arbitrator is not yet set
    let mut trade = trades().load(deps.storage, &info.sender.as_str())?;

    trade.arbitrator = arbitrator.clone();

    trades()
        .save(deps.storage, trade.trade.as_str(), &trade)
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "execute_update_trade_arbitrator")
        .add_attribute("tradeAddr", info.sender)
        .add_attribute("arbitrator", arbitrator)
        .add_attribute("timestamp", _env.block.time.seconds().to_string());

    Ok(res)
}

pub fn execute_update_last_traded(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, GuardError> {
    let trade = trades().load(deps.storage, &info.sender.as_str())?;

    let mut offer_model = OfferModel::may_load(deps.storage, &trade.offer_id);

    let offer = offer_model.update_last_traded(env.block.time.seconds());

    let res = Response::new()
        .add_attribute("action", "execute_update_last_traded")
        .add_attribute("tradeAddr", info.sender)
        .add_attribute("offer_id", &offer.id)
        .add_attribute("last_traded_at", &offer.last_traded_at.to_string());

    Ok(res)
}

pub fn create_arbitrator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    arbitrator: Addr,
    asset: FiatCurrency,
) -> Result<Response, GuardError> {
    assert_ownership(
        info.sender,
        Addr::unchecked("terra1rz4mcfwmqkgv7ss2tygpy79ffd33gh32as49j0"), // TODO move quorum address to constant
    )?;

    let index = arbitrator.clone().to_string() + &asset.to_string();

    arbitrators().save(
        deps.storage,
        &index,
        &Arbitrator {
            arbitrator: arbitrator.clone(),
            asset: asset.clone(),
        },
    )?;

    let res = Response::new()
        .add_attribute("action", "create_arbitrator")
        .add_attribute("arbitrator", arbitrator.to_string())
        .add_attribute("asset", asset.to_string())
        .add_attribute("timestamp", _env.block.time.seconds().to_string())
        .add_attribute(
            "numeric",
            ((_env.block.time.seconds() % 100) * (3 + 1) / (99 + 1)).to_string(),
        );

    Ok(res)
}

pub fn delete_arbitrator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    arbitrator: Addr,
    asset: FiatCurrency,
) -> Result<Response, GuardError> {
    assert_ownership(
        info.sender,
        Addr::unchecked("terra1rz4mcfwmqkgv7ss2tygpy79ffd33gh32as49j0"), // TODO move quorum address to constant
    )?;

    let index = arbitrator.clone().to_string() + &asset.to_string();

    arbitrators().remove(deps.storage, &index)?;

    let res = Response::new()
        .add_attribute("action", "delete_arbitrator")
        .add_attribute("arbitrator", arbitrator.to_string())
        .add_attribute("asset", asset.to_string());

    Ok(res)
}

pub fn update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferUpdateMsg,
) -> Result<Response, GuardError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let mut offer_model = OfferModel::may_load(deps.storage, &msg.id);

    assert_ownership(info.sender, offer_model.offer.owner.clone())?;

    let offer = offer_model.update(msg);

    let res = Response::new()
        .add_attribute("action", "update_offer")
        .add_attribute("id", offer.id.clone())
        .add_attribute("owner", offer.owner.to_string());

    Ok(res)
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, GuardError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_hub_addr(deps: Deps) -> StdResult<HubAddr> {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    Ok(hub_addr)
}

fn query_state(deps: Deps) -> StdResult<OffersCount> {
    let state = offers_count_read(deps.storage).load().unwrap();
    Ok(state)
}

pub fn load_offer_by_id(storage: &dyn Storage, id: String) -> StdResult<Offer> {
    let offer = offers()
        .may_load(storage, id.to_string())
        .unwrap_or_default()
        .unwrap();
    Ok(offer)
}

pub fn query_trades(
    env: Env,
    deps: Deps,
    user: Addr,
    index: TradesIndex,
    last_value: Option<Addr>,
    limit: u32,
) -> StdResult<Vec<TradeInfo>> {
    let mut trades_infos: Vec<TradeInfo> = vec![];

    // Pagination range (TODO pagination doesn't work with Addr as pk)
    let range_from = match last_value {
        Some(addr) => {
            let valid_addr = deps.api.addr_validate(addr.as_str()).unwrap();
            Some(Bound::exclusive(Vec::from(valid_addr.to_string())))
        }
        None => None,
    };

    // Select correct index for data lookup
    // * The `state<TradeState>` filter only supported for `user == arbitrator` queries
    let prefix = match index {
        TradesIndex::Seller => trades().idx.seller.prefix(user),
        TradesIndex::Buyer => trades().idx.buyer.prefix(user),
    };

    let trade_results: Vec<TradeAddr> = prefix
        .range(deps.storage, range_from, None, Order::Descending)
        .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
        .take(limit as usize)
        .collect();

    trade_results.iter().for_each(|t| {
        let trade_state: Trade = deps
            .querier
            .query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: t.trade.to_string(),
                msg: to_binary(&TradeQueryMsg::State {}).unwrap(),
            }))
            .unwrap();
        let offer: Offer = deps
            .querier
            .query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: trade_state.offer_contract.to_string(),
                msg: to_binary(&QueryMsg::Offer {
                    id: trade_state.offer_id.clone(),
                })
                .unwrap(),
            }))
            .unwrap();

        let current_time = env.block.time.seconds();

        let expired = current_time > trade_state.created_at + REQUEST_TIMEOUT; // TODO handle different possible expirations

        trades_infos.push(TradeInfo {
            trade: trade_state,
            offer,
            expired,
        })
    });
    Ok(trades_infos)
}

pub fn query_arbitrator(deps: Deps, arbitrator: Addr) -> StdResult<Vec<Arbitrator>> {
    let storage = deps.storage;

    let result = arbitrators()
        .idx
        .arbitrator
        .prefix(arbitrator)
        .range(storage, None, None, Order::Descending)
        .take(10)
        .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
        .collect();

    Ok(result)
}

pub fn query_arbitrators(
    deps: Deps,
    last_value: Option<String>,
    limit: u32,
) -> StdResult<Vec<Arbitrator>> {
    let storage = deps.storage;

    let range_from = match last_value {
        Some(addr) => Some(Bound::ExclusiveRaw(addr.into())),
        None => None,
    };

    let result = arbitrators()
        .range(storage, range_from, None, Order::Descending)
        .take(limit as usize)
        .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
        .collect();

    Ok(result)
}

pub fn query_arbitrator_asset(deps: Deps, asset: FiatCurrency) -> StdResult<Vec<Arbitrator>> {
    let storage = deps.storage;

    let result: Vec<Arbitrator> = arbitrators()
        .idx
        .asset
        .prefix(asset.clone().to_string())
        .range(storage, None, None, Order::Descending)
        .take(10)
        .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
        .collect();

    Ok(result)
}

pub fn query_arbitrator_random(
    deps: Deps,
    random_value: usize,
    asset: FiatCurrency,
) -> StdResult<Arbitrator> {
    assert_range_0_to_99(random_value).unwrap();

    let storage = deps.storage;

    let result: Vec<Arbitrator> = arbitrators()
        .idx
        .asset
        .prefix(asset.to_string())
        .range(storage, None, None, Order::Descending)
        .take(10)
        .flat_map(|item| item.and_then(|(_, arbitrator)| Ok(arbitrator)))
        .collect();

    let arbitrator_count = result.len();

    // Random range: 0..99
    // Mapped range: 0..result.len()-1
    // Formula is:
    // RandomValue * (MaxMappedRange + 1) / (MaxRandomRange + 1)
    let random_index = random_value * arbitrator_count / (99 + 1);

    Ok(result[random_index].clone())
}
