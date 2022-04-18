use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// This structure stores the main parameters for the staking contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// The LOCAL token contract address
    pub local_token_addr: Addr,
    /// The xLOCAL token contract address
    pub xlocal_token_addr: Addr,
}

/// ## Description
/// Stores the contract config at the given key
pub const CONFIG: Item<Config> = Item::new("config");

/// ## Description
/// This contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    /// The claims count, used e.g. for indexing
    pub claims_count: u64,
}

/// ## Description
/// Stores the contract state at the given key
pub const STATE: Item<State> = Item::new("state");

/// ## Description
/// Stores the claims indexedmap at the given key
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Claim {
    pub id: u64, // PK
    pub recipient: Addr,
    pub amount: Uint128,
    pub created_at: u64,
}

pub struct ClaimIndexes<'a> {
    // pk goes to second tuple element
    pub recipient: MultiIndex<'a, (Addr, Vec<u8>), Claim>,
}

impl<'a> IndexList<Claim> for ClaimIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Claim>> + '_> {
        let v: Vec<&dyn Index<Claim>> = vec![&self.recipient];
        Box::new(v.into_iter())
    }
}

pub fn claims<'a>() -> IndexedMap<'a, &'a str, Claim, ClaimIndexes<'a>> {
    let indexes = ClaimIndexes {
        recipient: MultiIndex::new(
            |d: &Claim, k: Vec<u8>| (d.recipient.clone(), k),
            "claims",
            "claims__recipient",
        ),
    };
    IndexedMap::new("claims", indexes)
}
