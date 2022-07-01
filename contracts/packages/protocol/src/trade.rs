use crate::currencies::FiatCurrency;
use cosmwasm_std::{Addr, Uint128};
use cw20::Denom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(Trade),
    FundEscrow {},
    RefundEscrow {},
    ReleaseEscrow {},
    DisputeEscrow {},
    FiatDeposited {},
    CancelRequest {},
    AcceptRequest {},
    RegisterHub {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    State {},
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trade {
    pub offer_id: String,
    pub denom: Denom,
    pub amount: Uint128,
    pub taker: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeData {
    pub addr: Addr,
    pub factory_addr: Addr,
    pub buyer: Addr,
    pub seller: Addr,
    pub arbitrator: Option<Addr>,
    pub offer_contract: Addr,
    pub offer_id: String,
    pub created_at: u64,
    pub denom: Denom,
    pub amount: Uint128,
    pub state: TradeState,
    pub asset: FiatCurrency,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradeState {
    RequestCreated,
    RequestAccepted,
    RequestCanceled,
    RequestExpired,
    EscrowFunded,
    EscrowRefunded,
    FiatDeposited,
    EscrowReleased,
    EscrowDisputed,
    SettledForMaker,
    SettledForTaker,
}

impl fmt::Display for TradeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
