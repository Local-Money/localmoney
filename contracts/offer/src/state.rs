use cosmwasm_std::{Order, StdResult, Storage};
use cosmwasm_storage::{
    bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{Config, Offer, State};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";
pub static OFFERS_KEY: &[u8] = b"offers";
pub static TRADES_KEY: &[u8] = b"trades";

pub fn config_storage(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state_storage(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

pub fn query_all_offers(
    storage: &dyn Storage,
    fiat_currency: FiatCurrency,
) -> StdResult<Vec<Offer>> {
    let offers: Vec<Offer> = bucket_read(storage, OFFERS_KEY)
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

pub fn trades_storage(storage: &mut dyn Storage, owner: String) -> Bucket<String> {
    let key: Vec<u8> = [TRADES_KEY, owner.as_bytes()].concat();
    Bucket::new(storage, key.as_slice())
}

pub fn trades_read(storage: &dyn Storage, owner: String) -> ReadonlyBucket<String> {
    let key: Vec<u8> = [TRADES_KEY, owner.as_bytes()].concat();
    bucket_read(storage, key.as_slice())
}

pub fn query_all_trades(storage: &dyn Storage, maker: String) -> StdResult<Vec<String>> {
    let trades: Vec<String> = trades_read(storage, maker.clone())
        .range(None, None, Order::Descending)
        .flat_map(|item| item.and_then(|(_, trade)| Ok(trade)))
        .collect();
    Ok(trades)
}
