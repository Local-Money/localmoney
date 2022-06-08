use cosmwasm_std::Addr;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// This structure describes the parameters used for creating a contract.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    /// The contract owner address
    pub owner: String,
    /// CW20 token code identifier
    pub token_code_id: u64,
    /// The LOCAL token contract address
    pub deposit_token_addr: String,
}

/// This structure describes the execute messages available in the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Receive receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received template.
    Receive(Cw20ReceiveMsg),
    Claim {
        claim_id: u64,
    },
}

/// This structure describes the query messages available in the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Config returns the contract configuration specified in a custom [`ConfigResponse`] structure
    Config {},
    TotalShares {},
    TotalDeposit {},
    TotalWarming {},
    Claims {
        recipient: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// The LOCAL token address
    pub deposit_token_addr: Addr,
    /// The xLOCAL token address
    pub share_token_addr: Addr,
}

/// This structure describes a migration message.
/// We currently take no arguments for migrations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// This structure describes a CW20 hook message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Deposits LOCAL in exchange for xLOCAL
    Enter {},
    /// Burns xLOCAL in exchange for LOCAL
    Leave {},
}
