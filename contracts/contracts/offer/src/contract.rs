use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Storage,
};
use cw_storage_plus::Bound;

use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::{HubAlreadyRegistered, Unauthorized};
use localterra_protocol::guards::{assert_min_g_max, assert_ownership, assert_range_0_to_99};
use localterra_protocol::hub_utils::{
    get_hub_admin, get_hub_config, register_hub_internal, HubAddr, HUB_ADDR,
};
use localterra_protocol::offer::{
    offers, Arbitrator, ExecuteMsg, InstantiateMsg, Offer, OfferModel, OfferMsg, OfferState,
    OfferUpdateMsg, OffersCount, QueryMsg,
};

use crate::state::{arbitrators, offers_count_read, offers_count_storage, trades};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    offers_count_storage(deps.storage)
        .save(&OffersCount { count: 0 })
        .unwrap();
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create { offer } => create_offer(deps, env, info, offer),
        ExecuteMsg::UpdateOffer { offer_update } => update_offer(deps, env, info, offer_update),
        ExecuteMsg::NewArbitrator { arbitrator, asset } => {
            create_arbitrator(deps, env, info, arbitrator, asset)
        }
        ExecuteMsg::DeleteArbitrator { arbitrator, asset } => {
            delete_arbitrator(deps, info, arbitrator, asset)
        }
        ExecuteMsg::UpdateTradeArbitrator { arbitrator } => {
            // TODO merge this call with the query random arbitrator call. LOCAL-660
            execute_update_trade_arbitrator(deps, env, info, arbitrator)
        }
        ExecuteMsg::UpdateLastTraded { offer_id } => {
            execute_update_last_traded(deps, env, info, offer_id)
        }
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
        ExecuteMsg::IncrementTradesCount { offer_id } => {
            increment_trades_count(deps, info, offer_id)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HubAddr {} => to_binary(&query_hub_addr(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps.storage, id)?),
        QueryMsg::Offers {
            owner,
            min,
            max,
            limit,
            order,
        } => to_binary(&OfferModel::query(deps, owner, min, max, limit, order)?),
        QueryMsg::OffersBy {
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            limit,
            order,
        } => to_binary(&OfferModel::query_by(
            deps.storage,
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            order,
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
) -> Result<Response, ContractError> {
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
) -> Result<Response, ContractError> {
    // TODO assert the calling contract can only update its own arbitrator and only if the arbitrator is not yet set. LOCAL-660
    let mut trade = trades().load(deps.storage, &info.sender.as_str()).unwrap();

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
    offer_id: String,
) -> Result<Response, ContractError> {
    let hub_config = get_hub_config(deps.as_ref());

    // Only allows to execute_update_last_traded if called by trade
    if info.sender.ne(&hub_config.trade_addr) {
        return Err(Unauthorized {
            owner: hub_config.trade_addr,
            caller: info.sender.clone(),
        });
    }

    let mut offer_model = OfferModel::may_load(deps.storage, &offer_id);

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
    env: Env,
    info: MessageInfo,
    arbitrator: Addr,
    asset: FiatCurrency,
) -> Result<Response, ContractError> {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    let admin = get_hub_admin(&deps.querier, hub_addr.addr.to_string());
    assert_ownership(info.sender, admin)?;

    let index = arbitrator.clone().to_string() + &asset.to_string();

    arbitrators()
        .save(
            deps.storage,
            &index,
            &Arbitrator {
                arbitrator: arbitrator.clone(),
                asset: asset.clone(),
            },
        )
        .unwrap();

    let res = Response::new()
        .add_attribute("action", "create_arbitrator")
        .add_attribute("arbitrator", arbitrator.to_string())
        .add_attribute("asset", asset.to_string())
        .add_attribute("timestamp", env.block.time.seconds().to_string())
        .add_attribute(
            "numeric",
            ((env.block.time.seconds() % 100) * (3 + 1) / (99 + 1)).to_string(),
        );

    Ok(res)
}

pub fn delete_arbitrator(
    deps: DepsMut,
    info: MessageInfo,
    arbitrator: Addr,
    asset: FiatCurrency,
) -> Result<Response, ContractError> {
    let hub_addr = HUB_ADDR.load(deps.storage).unwrap();
    let admin = get_hub_admin(&deps.querier, hub_addr.addr.to_string());
    assert_ownership(info.sender, admin)?;

    let index = arbitrator.clone().to_string() + &asset.to_string();

    arbitrators().remove(deps.storage, &index).unwrap();

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
) -> Result<Response, ContractError> {
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

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn increment_trades_count(
    deps: DepsMut,
    info: MessageInfo,
    offer_id: String,
) -> Result<Response, ContractError> {
    let hub_cfg = get_hub_config(deps.as_ref());

    //Check if caller is Trade Contract
    if info.sender.ne(&hub_cfg.trade_addr) {
        return Err(Unauthorized {
            owner: hub_cfg.trade_addr.clone(),
            caller: info.sender.clone(),
        });
    }

    //Increment trades_count
    let mut offer = load_offer_by_id(deps.storage, offer_id).unwrap();
    offer.trades_count += 1;
    OfferModel::store(deps.storage, &offer).unwrap();

    let res = Response::new()
        .add_attribute("offer_id", offer.id)
        .add_attribute("trades_count", offer.trades_count.to_string());
    Ok(res)
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
