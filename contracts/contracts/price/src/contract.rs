use std::ops::{Div, Mul};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint256,
};
use cw20::Denom;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::denom_utils::denom_to_string;
use localterra_protocol::errors::ContractError;
use localterra_protocol::errors::ContractError::HubAlreadyRegistered;
use localterra_protocol::guards::assert_ownership;
use localterra_protocol::hub_utils::{get_hub_admin, register_hub_internal};
use localterra_protocol::kujira::asset::{Asset, AssetInfo};
use localterra_protocol::kujira::denom::Denom as KujiraDenom;
use localterra_protocol::kujira::fin::QueryMsg as FinQueryMsg;
use localterra_protocol::kujira::fin::SimulationResponse;
use localterra_protocol::kujira::querier::KujiraQuerier;
use localterra_protocol::kujira::query::KujiraQuery;
use localterra_protocol::price::{
    CurrencyPrice, DenomFiatPrice, ExecuteMsg, PriceRoute, QueryMsg, DENOM_PRICE_ROUTE, FIAT_PRICE,
};
use localterra_protocol::profile::{InstantiateMsg, MigrateMsg};

// version info for migration info
pub const CONTRACT_NAME: &str = "localmoney.io:price";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new().add_attribute("action", "instantiate_price");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterHub {} => register_hub(deps, info),
        ExecuteMsg::UpdatePrices(prices) => update_prices(deps, env, prices),
        ExecuteMsg::RegisterPriceRouteForDenom { denom, route } => {
            register_price_route_for_denom(deps, info, denom, route)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { fiat, denom } => {
            to_binary(&query_fiat_price_for_denom(deps, fiat, denom)?)
        }
    }
}

fn register_hub(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    register_hub_internal(info.sender, deps.storage, HubAlreadyRegistered {})
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
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default()
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}
