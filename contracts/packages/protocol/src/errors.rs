use crate::offer::OfferState;
use crate::trade::TradeState;
use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
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
    #[error("Unauthorized2.")]
    UnauthorizedUser {
        caller: Addr,
        buyer: Addr,
        seller: Addr,
    },
    #[error("Hub Already Registered.")]
    HubAlreadyRegistered {},
    #[error("This trade has expired.")]
    TradeExpired {
        timeout: u64,
        expired_at: u64,
        created_at: u64,
    },
    #[error("Fund escrow error.")]
    FundEscrowError {
        required_amount: Uint128,
        sent_amount: Uint128,
    },
    #[error("Offer not found.")]
    OfferNotFound { offer_id: String },
    #[error("Invalid trade state change.")]
    InvalidTradeStateChange { from: TradeState, to: TradeState },
    #[error("Refund error: Not Expired")]
    RefundErrorNotExpired { message: String, trade: String },
    #[error("Distribution hasn't started yet.")]
    DistributionNotStarted {},
    #[error("Only past periods can be claimed.")]
    DistributionClaimInvalidPeriod {},
    #[error("Trade state is invalid.")]
    InvalidTradeState {
        current: TradeState,
        expected: TradeState,
    },
}
