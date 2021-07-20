use crate::error::ContractError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub trait Validate {
    fn validate(&self) -> Result<(), ContractError>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub bind_name: String,
    pub contract_name: String,
}

/// Simple validation of InstantiateMsg data
///
/// ### Example
///
/// ```rust
/// use {{contract.snake}}::msg::{InstantiateMsg, Validate};
/// pub fn instantiate(msg: InstantiateMsg){
///     let result = msg.validate();
///     todo!()
/// }
/// ```
#[allow(unused_mut)]
impl Validate for InstantiateMsg {
    fn validate(&self) -> Result<(), ContractError> {
        let mut invalid_fields: Vec<&str> = vec![];

        // validate the bind name
        if self.bind_name.is_empty() {
            invalid_fields.push("bind_name");
        }

        // validate the contract name
        if self.contract_name.is_empty() {
            invalid_fields.push("contract_name");
        }

        match invalid_fields.len() {
            0 => Ok(()),
            _ => Err(ContractError::InvalidFields {
                fields: invalid_fields.into_iter().map(|item| item.into()).collect(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // TODO: Add execute messages here
}

/// Simple validation of ExecuteMsg data
///
/// ### Example
///
/// ```rust
/// use {{contract.snake}}::msg::{ExecuteMsg, Validate};
/// pub fn execute(msg: ExecuteMsg){
///     let result = msg.validate();
///     todo!()
/// }
/// ```
#[allow(unused_mut)]
impl Validate for ExecuteMsg {
    fn validate(&self) -> Result<(), ContractError> {
        let mut invalid_fields: Vec<&str> = vec![];

        /* TODO: validate ExecuteMsg arguments here
        match self {
            // TODO: validate execute messages here
        }
        */

        match invalid_fields.len() {
            0 => Ok(()),
            _ => Err(ContractError::InvalidFields {
                fields: invalid_fields.into_iter().map(|item| item.into()).collect(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Get the contract info.
    GetContractInfo {},
    // TODO: Add query messages here
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    Migrate {},
}
