use crate::errors::ContractError;
use crate::offer::OfferType;
use crate::trade::{Trade, TradeState};
use cosmwasm_std::{Addr, StdError, Uint128};

pub fn assert_ownership(caller: Addr, owner: Addr) -> Result<(), ContractError> {
    if caller.eq(&owner) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized { owner, caller })
    }
}

pub fn assert_caller_is_buyer_or_seller(
    caller: Addr,
    buyer: Addr,
    seller: Addr,
) -> Result<(), ContractError> {
    if caller.eq(&buyer) || caller.eq(&seller) {
        Ok(())
    } else {
        Err(ContractError::UnauthorizedUser {
            caller,
            buyer,
            seller,
        })
    }
}

pub fn assert_trade_state_change_is_valid(
    from: TradeState,
    from_allowed: TradeState,
    to: TradeState,
) -> Result<(), ContractError> {
    if from.eq(&from_allowed) {
        Ok(())
    } else {
        Err(ContractError::InvalidTradeStateChange { from, to })
    }
}

pub fn assert_min_g_max(min: Uint128, max: Uint128) -> Result<(), ContractError> {
    if min >= max {
        Err(ContractError::Std(StdError::generic_err(
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
) -> Result<(), ContractError> {
    //Check that amount is inside Offer limits
    if amount > max || amount < min {
        return Err(ContractError::AmountError {
            amount,
            min_amount: min,
            max_amount: max,
        });
    } else {
        Ok(())
    }
}

pub fn assert_range_0_to_99(random_value: usize) -> Result<(), ContractError> {
    // No need to check `random_value < 0` since datatype is an unsigned integer
    if random_value > 99 {
        Err(ContractError::Std(StdError::generic_err(
            "Value out of range: 0..99.",
        )))
    } else {
        Ok(())
    }
}

pub fn trade_request_is_expired(block_time: u64, created_at: u64, expire_timer: u64) -> bool {
    block_time > created_at + expire_timer
}

pub fn assert_trade_state_and_type(
    trade: &Trade,
    offer_type: &OfferType,
) -> Result<(), ContractError> {
    if offer_type == &OfferType::Sell && trade.state == TradeState::RequestCreated {
        Ok(())
    } else if offer_type == &OfferType::Buy && trade.state == TradeState::RequestAccepted {
        Ok(())
    } else {
        Err(ContractError::Std(StdError::generic_err(
            "Incorrect sender funding the trade.", // TODO: use custom error. LOCAL-734
        )))
    }
}
