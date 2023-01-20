use crate::offer::OfferState;
use crate::trade::TradeState;
use cosmwasm_std::{Addr, Uint128, Uint256, Uint64};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    /// General Errors
    #[error("Unauthorized.")]
    Unauthorized { owner: Addr, caller: Addr },
    #[error("Unauthorized.")]
    UnauthorizedMultipleOwnership { owners: Vec<Addr>, caller: Addr },
    #[error("The parameter {0} is invalid. {1}", parameter, message.clone().unwrap_or_default())]
    InvalidParameter {
        parameter: String,
        message: Option<String>,
    },
    /// Hub Errors
    #[error("Hub already registered.")]
    HubAlreadyRegistered {},
    #[error("The sum of `chain_fee_pct`, `burn_fee_pct` and `warchest_fee_pct` must be less than {0}%.", max_platform_fee.to_string())]
    InvalidPlatformFee { max_platform_fee: Uint64 },
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
    #[error("Offer max amount: {max_amount:?} is above the trading limit: {trading_limit:?}.")]
    OfferMaxAboveTradingLimit {
        max_amount: Uint256,
        trading_limit: Uint256,
    },
    #[error("Offer not found.")]
    OfferNotFound { offer_id: String },
    #[error("Value out of range.")]
    ValueOutOfRange {
        value: usize,
        range_start: usize,
        range_end: usize,
    },
    /// Trade Errors
    #[error(
        "Fund escrow error. Required amount: {required_amount:?}, Sent amount: {sent_amount:?}."
    )]
    FundEscrowError {
        required_amount: Uint128,
        sent_amount: Uint128,
    },
    #[error("Dispute requested too early. Time to enable dispute: {time_to_dispute:?}")]
    PrematureDisputeRequest { time_to_dispute: u64 },
    #[error("Invalid denom. Expected: {expected:?}, Received: {received:?}.")]
    InvalidDenom { expected: String, received: String },
    #[error("Invalid price for denom. Must be greater than zero.")]
    InvalidPriceForDenom {},
    #[error("Invalid sender, must be Trade's buyer or seller.")]
    InvalidSender {
        sender: Addr,
        buyer: Addr,
        seller: Addr,
    },
    #[error("Invalid trade amount. Amount: {amount:?}. Min: {min_amount:?}. Max: {max_amount:?}.")]
    InvalidTradeAmount {
        amount: Uint256,
        min_amount: Uint256,
        max_amount: Uint256,
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
    #[error("Swap Error: received amount is less than expected.")]
    SwapErrorInvalidAmount {},
    #[error("Swap Error: missing denom.")]
    SwapErrorMissingDenom { expected_denom: String },
    #[error("Unknown reply id: {reply_id:?}")]
    UnknownReplyId { reply_id: u64 },
    /// Profile Errors
    #[error("Active offers limit reached. Limit: {limit:?}.")]
    ActiveOffersLimitReached { limit: u8 },
    #[error("Active trades limit reached. Limit: {limit:?}.")]
    ActiveTradesLimitReached { limit: u8 },
}
