use crate::constants::{MAX_ITEMS_PER_PAGE, MIN_ITEMS_PER_PAGE, OFFER_DESCRIPTION_LIMIT};
use crate::errors::ContractError;
use crate::offer::OfferType;
use crate::trade::{Trade, TradeState};
use cosmwasm_std::{Addr, Uint128, Uint256};
use cw2::ContractVersion;

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

pub fn assert_trade_state_change(
    from: TradeState,
    allowed_states: Vec<TradeState>,
    to: TradeState,
) -> Result<(), ContractError> {
    if allowed_states.contains(&from) {
        Ok(())
    } else {
        Err(ContractError::InvalidTradeStateChange { from, to })
    }
}

// Asserts that min value is lower than max value
pub fn assert_min_g_max(min: Uint128, max: Uint128) -> Result<(), ContractError> {
    if min >= max {
        Err(ContractError::InvalidMinMax { min, max })
    } else {
        Ok(())
    }
}

pub fn assert_offer_max_inside_trading_limit(
    max_amount: Uint256,
    trading_limit: Uint256,
) -> Result<(), ContractError> {
    if max_amount > trading_limit {
        Err(ContractError::OfferMaxAboveTradingLimit {
            max_amount,
            trading_limit,
        })
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

pub fn assert_offer_description_valid(description: Option<String>) -> Result<(), ContractError> {
    let description = description.unwrap_or(String::new());
    return if description.len() > OFFER_DESCRIPTION_LIMIT {
        let mut message = "The description can not be longer than ".to_string();
        message.push_str(OFFER_DESCRIPTION_LIMIT.to_string().as_str());
        message.push_str(" characters.");

        Err(ContractError::InvalidParameter {
            parameter: "description".to_string(),
            message: Some(message),
        })
    } else {
        Ok(())
    };
}

pub fn assert_migration_parameters(
    previous_contract_version: ContractVersion,
    contract_name: String,
    contract_version: &str,
) -> Result<(), ContractError> {
    let previous_version = previous_contract_version.version.as_str();

    if previous_contract_version.contract != contract_name {
        return Err(ContractError::InvalidParameter {
            parameter: "CONTRACT_NAME".to_string(),
            message: Some("Can only upgrade from same type.".to_string()),
        });
    }

    if previous_version >= contract_version {
        let message = format!(
            "The new version of the contract ({}) must be greater than the previous one ({}).",
            contract_version, previous_version
        );
        return Err(ContractError::InvalidParameter {
            parameter: "CONTRACT_VERSION".to_string(),
            message: Some(message),
        });
    }

    Ok(())
}

pub fn validate_min_max_items_per_page(limit: u32) -> u32 {
    limit.max(MIN_ITEMS_PER_PAGE).min(MAX_ITEMS_PER_PAGE)
}
