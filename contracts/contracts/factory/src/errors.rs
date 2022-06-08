use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FactoryError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
}
