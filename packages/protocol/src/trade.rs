use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub offer_id: u64,
    pub ust_amount: Uint128,
    pub counterparty: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    FundEscrow,
    Refund,
    Release,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    State {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub factory_addr: Addr,
    pub recipient: Addr,
    pub sender: Addr,
    pub offer_contract: Addr,
    pub offer_id: u64,
    pub expire_height: u64,
    pub ust_amount: Uint128,
    pub state: TradeState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TradeState {
    Canceled,
    Closed,
    Created,
    EscrowFunded,
}
