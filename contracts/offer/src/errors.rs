use crate::state::OfferState;
use cosmwasm_std::{HumanAddr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OfferError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Invalid state change.")]
    InvalidStateChange { from: OfferState, to: OfferState },
    #[error("Unauthorized.")]
    Unauthorized { owner: HumanAddr, caller: HumanAddr },
}
