use std::ops::{Div, Mul};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
    Uint128, Uint256,
};
use cw20::Denom;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::denom_utils::denom_to_string;
use localterra_protocol::kujira::asset::{Asset, AssetInfo};
use localterra_protocol::kujira::denom::Denom as KujiraDenom;
use localterra_protocol::kujira::fin::{QueryMsg as FinQueryMsg, SimulationResponse};
use localterra_protocol::kujira::querier::KujiraQuerier;
use localterra_protocol::kujira::query::KujiraQuery;

use crate::state::{offers_count_read, offers_count_storage};
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::HubAlreadyRegistered;
use localterra_protocol::guards::{assert_min_g_max, assert_ownership};
use localterra_protocol::hub_utils::{get_hub_admin, get_hub_config, register_hub_internal};
use localterra_protocol::offer::{
    offers, CurrencyPrice, DenomFiatPrice, ExecuteMsg, InstantiateMsg, MigrateMsg, Offer,
    OfferModel, OfferMsg, OfferResponse, OfferState, OfferUpdateMsg, OffersCount, PriceRoute,
    QueryMsg, DENOM_PRICE_ROUTE, FIAT_PRICE,
};
use localterra_protocol::profile::{load_profile, update_profile_msg};

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
    let res = Response::new().add_attribute("action", "instantiate_offer");
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
        ExecuteMsg::Create { offer } => create_offer(deps, env, info, offer),
        ExecuteMsg::UpdateOffer { offer_update } => update_offer(deps, env, info, offer_update),
        ExecuteMsg::UpdatePrices(prices) => update_prices(deps, env, prices),
        ExecuteMsg::RegisterPriceRouteForDenom { denom, route } => {
            register_price_route_for_denom(deps, info, denom, route)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Offer { id } => to_binary(&load_offer_by_id(deps, id)?),
        QueryMsg::OffersBy {
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            order,
            limit,
        } => to_binary(&OfferModel::query_by(
            deps,
            offer_type,
            fiat_currency,
            denom,
            min,
            max,
            limit,
            order,
        )?),
        QueryMsg::OffersByOwner { owner, limit } => {
            to_binary(&OfferModel::query_by_owner(deps, owner, limit)?)
        }
        QueryMsg::Price { fiat: _, denom: _ } => {
            todo!()
            // to_binary(&query_fiat_price_for_denom(deps, fiat, denom)?)
        }
    }
}

pub fn create_offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: OfferMsg,
) -> Result<Response, ContractError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    // Load offers count to create the next sequential id, maybe we can switch to a hash based id in the future.
    let mut offers_count = offers_count_storage(deps.storage).load().unwrap();
    offers_count.count += 1;
    let offer_id = [msg.rate.clone().to_string(), offers_count.count.to_string()].join("_");

    let hub_config = get_hub_config(deps.as_ref());

    let update_profile_msg = update_profile_msg(
        hub_config.profile_addr.to_string(),
        info.sender.clone(),
        msg.owner_contact.clone(),
        msg.owner_encryption_key.clone(),
    );

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
        },
    )
    .offer;

    offers_count_storage(deps.storage)
        .save(&offers_count)
        .unwrap();

    let res = Response::new()
        .add_submessage(update_profile_msg)
        .add_attribute("action", "create_offer")
        .add_attribute("type", offer.offer_type.to_string())
        .add_attribute("id", offer.id.to_string())
        .add_attribute("rate", offer.rate.to_string())
        .add_attribute("min_amount", offer.min_amount.to_string())
        .add_attribute("max_amount", offer.max_amount.to_string())
        .add_attribute("owner", offer.owner);
    Ok(res)
}

pub fn update_offer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OfferUpdateMsg,
) -> Result<Response, ContractError> {
    assert_min_g_max(msg.min_amount, msg.max_amount)?;

    let hub_config = get_hub_config(deps.as_ref());
    let mut offer_model = OfferModel::may_load(deps.storage, &msg.id);

    assert_ownership(info.sender.clone(), offer_model.offer.owner.clone())?;

    let mut sub_msgs: Vec<SubMsg> = Vec::new();
    if msg.owner_contact.is_some() && msg.owner_encryption_key.is_some() {
        sub_msgs.push(update_profile_msg(
            hub_config.profile_addr.to_string(),
            info.sender.clone(),
            msg.owner_contact.clone().unwrap(),
            msg.owner_encryption_key.clone().unwrap(),
        ));
    }

    let offer = offer_model.update(msg);

    let res = Response::new()
        .add_submessages(sub_msgs)
        .add_attribute("action", "update_offer")
        .add_attribute("id", offer.id.clone())
        .add_attribute("owner", offer.owner.to_string());

    Ok(res)
}

pub fn update_prices(
    deps: DepsMut,
    env: Env,
    prices: Vec<CurrencyPrice>,
) -> Result<Response, ContractError> {
    // TODO: Permissions check
    let mut attrs: Vec<(&str, String)> = vec![("action", "update_prices".to_string())];
    prices.iter().for_each(|price| {
        // Load existing object or default
        let path = FIAT_PRICE.key(price.currency.to_string().as_str());
        let mut currency_price = path
            .load(deps.storage)
            .unwrap_or(CurrencyPrice::new(price.currency.clone()));

        // Update price
        currency_price.usd_price = price.usd_price;
        currency_price.updated_at = env.block.time.seconds();
        path.save(deps.storage, &currency_price).unwrap();
        attrs.push(("currency", price.currency.to_string()));
        attrs.push(("usd_price", price.usd_price.to_string()));
    });
    let res = Response::new().add_attributes(attrs);
    Ok(res)
}

pub fn register_price_route_for_denom(
    deps: DepsMut,
    info: MessageInfo,
    denom: Denom,
    route: Vec<PriceRoute>,
) -> Result<Response, ContractError> {
    let admin = get_hub_admin(deps.as_ref()).addr;
    assert_ownership(info.sender, admin)?;

    let denom_str = denom_to_string(&denom.clone());
    DENOM_PRICE_ROUTE
        .save(deps.storage, denom_str.as_str(), &route)
        .unwrap();

    let mut attrs = vec![
        ("action".to_string(), "register_price".to_string()),
        ("denom".to_string(), denom_str),
    ];
    route
        .iter()
        .for_each(|step| attrs.push(("route_step".to_string(), step.to_string())));
    let res = Response::default().add_attributes(attrs);
    Ok(res)
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
}

fn query_state(deps: Deps) -> StdResult<OffersCount> {
    let state = offers_count_read(deps.storage).load().unwrap();
    Ok(state)
}

pub fn load_offer_by_id(deps: Deps, id: String) -> StdResult<OfferResponse> {
    let hub_config = get_hub_config(deps);
    let offer = offers()
        .may_load(deps.storage, id.to_string())
        .unwrap_or_default()
        .unwrap();
    let profile = load_profile(
        &deps.querier,
        hub_config.profile_addr.to_string(),
        offer.owner.clone(),
    )
    .unwrap();
    Ok(OfferResponse { offer, profile })
}

pub fn query_fiat_price_for_denom(
    deps: Deps<KujiraQuery>,
    fiat: FiatCurrency,
    denom: Denom,
) -> StdResult<DenomFiatPrice> {
    let fiat_price = &FIAT_PRICE.load(deps.storage, fiat.to_string().as_str())?;
    let kq = KujiraQuerier::new(&deps.querier);
    let atom_usd_price = kq.query_exchange_rate("ATOM".to_string()).unwrap();
    let amount = Uint128::new(1_000_000u128);
    let denom_str = denom_to_string(&denom.clone());
    let denom_price_route = &DENOM_PRICE_ROUTE
        .load(deps.storage, denom_str.as_str())
        .unwrap();
    let denom_atom = denom_price_route
        .iter()
        .fold(Uint256::from(1u128), |price, route| {
            let denom_price_result: SimulationResponse = deps
                .querier
                .query_wasm_smart(
                    route.pool.clone(),
                    &FinQueryMsg::Simulation {
                        offer_asset: Asset {
                            info: AssetInfo::NativeToken {
                                denom: KujiraDenom::from(denom_to_string(&route.offer_asset)),
                            },
                            amount,
                        },
                    },
                )
                .unwrap();
            price * denom_price_result.return_amount
        });
    let fiat_usd = Uint256::from(fiat_price.usd_price);
    let atom_usd = Uint256::from(Uint128::new(1_000_000u128).mul(atom_usd_price.rate));
    let decimal_places = 1_000_000_000_000u128;
    let denom_fiat_price = fiat_usd
        .mul(&atom_usd)
        .mul(&denom_atom)
        .div(Uint256::from(decimal_places));
    Ok(DenomFiatPrice {
        denom: denom.clone(),
        fiat: fiat.clone(),
        price: denom_fiat_price,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    //TODO temporary
    offers_count_storage(deps.storage)
        .save(&OffersCount { count: 0 })
        .unwrap();
    Ok(Response::default())
}
