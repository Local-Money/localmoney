use cosmwasm_std::{Deps, Order, StdResult, Storage};
use cosmwasm_storage::{bucket_read, singleton, singleton_read, ReadonlySingleton, Singleton};
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{Config, Offer, CONFIG_KEY, OFFERS_KEY};

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn query_all_offers(deps: Deps, fiat_currency: FiatCurrency) -> StdResult<Vec<Offer>> {
    let offers: Vec<Offer> = bucket_read(deps.storage, OFFERS_KEY)
        .range(None, None, Order::Descending)
        .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
        .collect();

    let result: Vec<Offer> = offers
        .iter()
        .filter(|offer| offer.fiat_currency == fiat_currency)
        .cloned()
        .collect();

    Ok(result)
}
