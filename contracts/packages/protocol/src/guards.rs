use crate::errors::ContractError;
use crate::offer::OfferType;
use crate::trade::{Trade, TradeState};
use cosmwasm_std::{Addr, Uint128};

pub fn assert_multiple_ownership(caller: Addr, owners: Vec<Addr>) -> Result<(), ContractError> {
    if owners.contains(&caller) {
        Ok(())
    } else {
        Err(ContractError::UnauthorizedMultipleOwnership { owners, caller })
    }
}

pub fn assert_ownership(caller: Addr, owner: Addr) -> Result<(), ContractError> {
    if caller.eq(&owner) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized { owner, caller })
    }
}

pub fn assert_sender_is_buyer_or_seller(
    sender: Addr,
    buyer: Addr,
    seller: Addr,
) -> Result<(), ContractError> {
    if sender.eq(&buyer) || sender.eq(&seller) {
        Ok(())
    } else {
        Err(ContractError::InvalidSender {
            sender,
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
        Err(ContractError::InvalidMinMax { min, max })
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
        return Err(ContractError::InvalidOfferAmount {
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
        Err(ContractError::ValueOutOfRange {
            value: random_value,
            range_start: 0,
            range_end: 99,
        })
    } else {
        Ok(())
    }
}

pub fn assert_trade_state_and_type(
    trade: &Trade,
    offer_type: &OfferType,
) -> Result<(), ContractError> {
    if offer_type == &OfferType::Sell && trade.get_state() == TradeState::RequestCreated {
        Ok(())
    } else if offer_type == &OfferType::Buy && trade.get_state() == TradeState::RequestAccepted {
        Ok(())
    } else {
        Err(ContractError::InvalidTradeState {
            current: trade.get_state(),
            expected: match offer_type {
                OfferType::Buy => TradeState::RequestAccepted,
                OfferType::Sell => TradeState::RequestCreated,
            },
        })
    }
}
