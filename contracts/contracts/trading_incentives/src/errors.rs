use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingIncentivesError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
}
