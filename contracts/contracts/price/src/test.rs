use cosmwasm_std::{Decimal, Uint128, Uint256};
use std::ops::Mul;

#[test]
fn test() {
    let denom_fiat_price = Uint256::from_u128(10u128);
    let offer_rate: Uint128 = Uint128::new(199u128);
    let offer_rate = Decimal::from_ratio(offer_rate.clone(), Uint128::new(100u128));
    let offer_rate = Uint256::from(Uint128::new(100u128).mul(offer_rate));
    let denom_final_price = denom_fiat_price
        .checked_mul(offer_rate)
        .unwrap()
        .checked_div(Uint256::from_u128(100u128))
        .unwrap_or(Uint256::MAX);
    assert_eq!(denom_final_price, Uint256::from_u128(19u128));
}
