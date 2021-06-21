//TODO:
/*
use cosmwasm_std::{Api, Coin, Querier, StdResult, Storage, Uint128};

static DECIMAL_FRACTION: Uint128 = Uint128(1_000_000_000_000_000_000u128);

pub fn compute_tax<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    coin: &Coin,
) -> StdResult<Uint128> {
    return StdResult::Ok(Uint128::zero());
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate= (terra_querier.query_tax_rate()?).rate;
    let tax_cap= (terra_querier.query_tax_cap(coin.denom.to_string())?).cap;
    Ok(std::cmp::min(
        (coin.amount
            - coin.amount.multiply_ratio(
                DECIMAL_FRACTION,
                DECIMAL_FRACTION * tax_rate.into() + DECIMAL_FRACTION,
            ))?,
        tax_cap.into(),
    ))
}

pub fn deduct_tax<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    coin: Coin,
) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: (coin.amount - Uint128::zero())?,
    })
}
*/
