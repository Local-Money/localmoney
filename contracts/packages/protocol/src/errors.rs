use crate::offer::OfferState;
use crate::trade::TradeState;
use cosmwasm_std::{Addr, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    /// General Errors
    #[error("Unauthorized.")]
    Unauthorized { owner: Addr, caller: Addr },
    #[error("Unauthorized.")]
    UnauthorizedMultipleOwnership { owners: Vec<Addr>, caller: Addr },
    #[error("Some parameter is missing")]
    MissingParameter {
        missing: String,
        message: Option<String>,
    },
    /// Hub Errors
    #[error("Hub already registered.")]
    HubAlreadyRegistered {},
    /// Offer Errors
    #[error("Min amount must be greater than Max amount.")]
    InvalidMinMax { min: Uint128, max: Uint128 },
    #[error("Amount is outside of offer amount range.")]
    InvalidOfferAmount {
        amount: Uint128,
        min_amount: Uint128,
        max_amount: Uint128,
    },
    #[error("Invalid state change.")]
    InvalidOfferStateChange { from: OfferState, to: OfferState },
    #[error("Offer not found.")]
    OfferNotFound { offer_id: String },
    #[error("Value out of range.")]
    ValueOutOfRange {
        value: usize,
        range_start: usize,
        range_end: usize,
    },
    /// Trade Errors
    #[error("Fund escrow error.")]
    FundEscrowError {
        required_amount: Uint128,
        sent_amount: Uint128,
    },
    #[error("Invalid sender, must be Trade's buyer or seller.")]
    InvalidSender {
        sender: Addr,
        buyer: Addr,
        seller: Addr,
    },
    #[error("Trade state is invalid.")]
    InvalidTradeState {
        current: TradeState,
        expected: TradeState,
    },
    #[error("Invalid trade state change.")]
    InvalidTradeStateChange { from: TradeState, to: TradeState },
    #[error("Refund error: Not Expired")]
    RefundErrorNotExpired { message: String, trade: String },
    #[error("This trade has expired.")]
    TradeExpired { expired_at: u64, created_at: u64 },
    /// TradingIncentives Errors
    #[error("Only past periods can be claimed.")]
    DistributionClaimInvalidPeriod {},
    #[error("Distribution hasn't started yet.")]
    DistributionNotStarted {},
}
