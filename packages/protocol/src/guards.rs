use crate::errors::OfferError;
use cosmwasm_std::{Addr, StdError, StdResult, Uint128};

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
