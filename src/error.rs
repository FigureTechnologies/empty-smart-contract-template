use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid fields: {fields:?}")]
    InvalidFields { fields: Vec<String> },
}

impl From<ContractError> for StdError {
    fn from(error: ContractError) -> Self {
        StdError::GenericErr {
            msg: error.to_string(),
        }
    }
}
