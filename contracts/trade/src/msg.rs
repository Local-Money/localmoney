use crate::state::State;
use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub offer_contract: Addr,
    pub fee_collector: Addr,
    pub offer_id: u64,
    pub ust_amount: Uint128,
    pub final_asset: Option<String>,
    //TODO: Move to Offer contract.
    pub terraswap_factory: Option<Addr>,
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
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferMsg {
    LoadOffer { id: u64 },
}

pub type ConfigResponse = State;
