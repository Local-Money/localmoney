use crate::offer::OfferState;
use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Invalid state change.")]
    InvalidStateChange { from: OfferState, to: OfferState },
    #[error("Amount is outside of offer amount range.")]
    AmountError {
        amount: Uint128,
        min_amount: Uint128,
        max_amount: Uint128,
    },
    #[error("Unauthorized.")]
    Unauthorized { owner: Addr, caller: Addr },
    #[error("Governance not found.")]
    GovernanceNotFound { gov_addr: Addr },
    #[error("Invalid reply message id.")]
    InvalidReply {},
}
