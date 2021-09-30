use cosmwasm_std::{Decimal, Uint128};

pub const DECIMAL_FRACTIONAL: Uint128 = Uint128::new(1_000_000_000u128);

/// return a / b
#[allow(dead_code)]
pub fn decimal_division(a: Decimal, b: Decimal) -> Decimal {
    Decimal::from_ratio(DECIMAL_FRACTIONAL * a, b * DECIMAL_FRACTIONAL)
}

#[allow(dead_code)]
pub fn decimal_subtraction(a: Decimal, b: Decimal) -> Decimal {
    Decimal::from_ratio(
        (DECIMAL_FRACTIONAL * a)
            .checked_sub(DECIMAL_FRACTIONAL * b)
            .unwrap(),
        DECIMAL_FRACTIONAL,
    )
}
