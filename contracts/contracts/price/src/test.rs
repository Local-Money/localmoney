use cosmwasm_std::{Uint128, Uint256};
use localmoney_protocol::trade::calc_denom_fiat_price;

#[test]
fn test() {
    let offer_rate: Uint128 = Uint128::new(199u128);
    let denom_fiat_price = Uint256::from_u128(10u128);
    let denom_final_price = calc_denom_fiat_price(offer_rate, denom_fiat_price);
    assert_eq!(denom_final_price, Uint256::from_u128(19u128));
}
