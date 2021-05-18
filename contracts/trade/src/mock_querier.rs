use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_slice, to_binary, Api, Coin, Decimal, Extern, HumanAddr, Querier, QuerierResult,
    QueryRequest, SystemError, Uint128, WasmQuery,
};
use serde::{Deserialize, Serialize};
use terra_cosmwasm::{TaxCapResponse, TaxRateResponse, TerraQuery, TerraQueryWrapper};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OfferResponse {
    pub owner: HumanAddr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FiatCurrency {
    Cop,
    Brl,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OfferState {
    Active,
    Paused,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TradeState {
    Canceled,
    Closed,
    Created,
    EscrowFunded,
}

pub fn mock_dependencies_custom(
    canonical_length: usize,
    contract_balance: &[Coin],
) -> Extern<MockStorage, MockApi, WasmMockQuerier> {
    let contract_addr = HumanAddr::from(MOCK_CONTRACT_ADDR);
    let custom_querier: WasmMockQuerier = WasmMockQuerier::new(
        MockQuerier::new(&[(&contract_addr, contract_balance)]),
        canonical_length,
        MockApi::new(canonical_length),
    );

    Extern {
        storage: MockStorage::default(),
        api: MockApi::new(canonical_length),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}
pub fn test_offer() -> OfferResponse {
    return OfferResponse {
        owner: HumanAddr::from("offer_owner"),
        offer_type: OfferType::Buy,
        fiat_currency: FiatCurrency::Brl,
        min_amount: Uint128(500000),
        max_amount: Uint128(2000000),
        state: OfferState::Active,
    };
}
impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr,
                msg: _msg,
            }) => {
                return if contract_addr == &HumanAddr::from("offer") {
                    let offer = test_offer();
                    return Ok(to_binary(&offer));
                } else {
                    self.base.handle_query(request)
                };
            }
            QueryRequest::Custom(TerraQueryWrapper {
                route: _route,
                query_data,
            }) => match query_data {
                TerraQuery::TaxRate {} => {
                    let res = TaxRateResponse {
                        rate: Decimal::percent(1),
                    };
                    Ok(to_binary(&res))
                }
                TerraQuery::TaxCap { denom: _ } => {
                    let cap = Uint128(1u128);
                    let res = TaxCapResponse { cap };
                    Ok(to_binary(&res))
                }
                _ => panic!("DO NOT ENTER HERE"),
            },
            _ => self.base.handle_query(request),
        }
    }
}
impl WasmMockQuerier {
    pub fn new<A: Api>(
        base: MockQuerier<TerraQueryWrapper>,
        _canonical_length: usize,
        _api: A,
    ) -> Self {
        WasmMockQuerier { base }
    }
}
