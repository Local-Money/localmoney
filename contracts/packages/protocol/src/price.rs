use crate::{currencies::FiatCurrency, denom_utils::denom_to_string};
use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, StdResult, Uint128, Uint256};
use cw20::Denom;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterHub {},
    RegisterPriceRouteForDenom {
        denom: Denom,
        route: Vec<PriceRoute>,
    },
    UpdatePrices(Vec<CurrencyPrice>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Price { fiat: FiatCurrency, denom: Denom },
}

pub const FIAT_PRICE: Map<&str, CurrencyPrice> = Map::new("fiat_price");
pub const DENOM_PRICE_ROUTE: Map<&str, Vec<PriceRoute>> = Map::new("denom_price_route");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CurrencyPrice {
    pub currency: FiatCurrency,
    pub usd_price: Uint128,
    pub updated_at: u64,
}

impl CurrencyPrice {
    pub fn new(currency: FiatCurrency) -> CurrencyPrice {
        CurrencyPrice {
            currency,
            usd_price: Uint128::zero(),
            updated_at: 0u64,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomFiatPrice {
    pub denom: Denom,
    pub fiat: FiatCurrency,
    pub price: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceRoute {
    pub pool: Addr,
    pub offer_asset: Denom,
}

impl fmt::Display for PriceRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let denom_str = denom_to_string(&self.offer_asset);
        write!(
            f,
            "pool: {}, offer_asset: {}",
            self.pool.to_string(),
            denom_str
        )
    }
}

pub fn query_fiat_price_for_denom<T: CustomQuery>(
    querier: &QuerierWrapper<T>,
    denom: Denom,
    fiat: FiatCurrency,
    price_contract: String,
) -> StdResult<DenomFiatPrice> {
    querier.query_wasm_smart(price_contract, &QueryMsg::Price { fiat, denom })
}
