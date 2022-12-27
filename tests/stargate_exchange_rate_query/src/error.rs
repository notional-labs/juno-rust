use cosmwasm_std::StdError;
use prost::DecodeError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Decode(#[from] DecodeError),
}
