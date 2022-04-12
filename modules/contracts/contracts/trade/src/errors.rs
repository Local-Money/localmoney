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
        expire_timer: u64,
        expired_at: u64,
        created_at: u64,
    },
    #[error("Failed to instantiate contract.")]
    InstantiationError { message: String },
    #[error("Offer not found.")]
    OfferNotFound { offer_id: u64 },
    #[error("Refund error.")]
    RefundError { message: String, trade: String },
    #[error("Release error.")]
    ReleaseError { message: String },
    #[error("Swap error.")]
    SwapError {
        required_amount: Uint128,
        returned_amount: Uint128,
    },
    #[error("Fund escrow error.")]
    FundEscrowError {
        required_amount: Uint128,
        sent_amount: Uint128,
    },
    #[error("Escrow already funded.")]
    AlreadyFundedError {},
    #[error("Unauthorized.")]
    Unauthorized {
        owner: Addr,
        arbitrator: Addr,
        caller: Addr,
    },
    #[error("Unauthorized Dispute.")]
    UnauthorizedDispute {
        seller: Addr,
        buyer: Addr,
        caller: Addr,
    },
}
