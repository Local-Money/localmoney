use crate::errors::OfferError;
use cosmwasm_std::{Addr, StdError, Uint128};

pub fn assert_ownership(caller: Addr, owner: Addr) -> Result<(), OfferError> {
    if caller.eq(&owner) {
        Ok(())
    } else {
        Err(OfferError::Unauthorized { owner, caller })
    }
}

pub fn assert_min_g_max(min: Uint128, max: Uint128) -> Result<(), OfferError> {
    if min >= max {
        Err(OfferError::Std(StdError::generic_err(
            "Min amount must be greater than Max amount.",
        )))
    } else {
        Ok(())
    }
}

pub fn assert_range_0_to_99(random_value: usize) -> Result<(), OfferError> {
    // No need to check `random_value < 0` since datatype is an unsigned integer
    if (random_value > 99) {
        Err(OfferError::Std(StdError::generic_err(
            "Value out of range: 0..99.",
        )))
    } else {
        Ok(())
    }
}
