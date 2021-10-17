use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradeError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Amount is outside of offer amount range.")]
    AmountError {
        amount: Uint128,
        min_amount: Uint128,
        max_amount: Uint128,
    },
    #[error("Failed to execute contract.")]
    ExecutionError { message: String },
    #[error("This trade has expired.")]
    Expired {
        current_height: u64,
        expire_height: u64,
    },
    #[error("Failed to instantiate contract.")]
    InstantiationError { message: String },
    #[error("Offer not found.")]
    OfferNotFound { offer_id: u64 },
    #[error("Refund error.")]
    RefundError { message: String },
    #[error("Release error.")]
    ReleaseError { message: String },
    #[error("Swap error.")]
    SwapError {
        required_amount: Uint128,
        returned_amount: Uint128,
    },
    #[error("Unauthorized.")]
    Unauthorized { owner: Addr, caller: Addr },
}
