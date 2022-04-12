use crate::errors::GuardError;
use crate::offer::OfferType;
use crate::trade::{TradeData, TradeState};
use cosmwasm_std::{Addr, StdError, Uint128};

pub fn assert_ownership(caller: Addr, owner: Addr) -> Result<(), GuardError> {
    if caller.eq(&owner) {
        Ok(())
    } else {
        Err(GuardError::Unauthorized { owner, caller })
    }
}

pub fn assert_min_g_max(min: Uint128, max: Uint128) -> Result<(), GuardError> {
    if min >= max {
        Err(GuardError::Std(StdError::generic_err(
            "Min amount must be greater than Max amount.",
        )))
    } else {
        Ok(())
    }
}

pub fn assert_value_in_range(
    min: Uint128,
    max: Uint128,
    amount: Uint128,
) -> Result<(), GuardError> {
    //Check that ust_amount is inside Offer limits
    if amount > max || amount < min {
        return Err(GuardError::AmountError {
            amount,
            min_amount: min,
            max_amount: max,
        });
    } else {
        Ok(())
    }
}

pub fn assert_range_0_to_99(random_value: usize) -> Result<(), GuardError> {
    // No need to check `random_value < 0` since datatype is an unsigned integer
    if random_value > 99 {
        Err(GuardError::Std(StdError::generic_err(
            "Value out of range: 0..99.",
        )))
    } else {
        Ok(())
    }
}

pub fn trade_request_is_expired(block_time: u64, created_at: u64, expire_timer: u64) -> bool {
    block_time > created_at + expire_timer
}

pub fn assert_trade_state_for_sender(
    sender: Addr,
    trade: &TradeData,
    offer_type: &OfferType,
) -> Result<(), GuardError> {
    // sender == maker == seller
    if offer_type == &OfferType::Sell
        && &sender == &trade.seller
        && trade.state == TradeState::RequestCreated
    {
        Ok(())
    }
    // sender == taker == seller
    else if offer_type == &OfferType::Buy
        && &sender == &trade.seller
        && trade.state == TradeState::RequestAccepted
    {
        Ok(())
    } else {
        Err(GuardError::Std(StdError::generic_err(
            "Incorrect sender funding the trade.", // TODO use costum error and return the funds
        )))
    }
}
