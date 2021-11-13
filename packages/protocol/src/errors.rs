use crate::offer::OfferState;
use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfferError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Invalid state change.")]
    InvalidStateChange { from: OfferState, to: OfferState },
    #[error("Unauthorized.")]
    Unauthorized { owner: Addr, caller: Addr },
    #[error("Governance not found.")]
    GovernanceNotFound { gov_addr: Addr },
    #[error("Invalid reply message id.")]
    InvalidReply {},
}
