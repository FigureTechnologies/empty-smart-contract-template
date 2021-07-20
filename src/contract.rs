use crate::contract_info::{get_contract_info, set_contract_info, ContractInfo};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, Validate};
use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use provwasm_std::ProvenanceMsg;

pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// smart contract initialization entrypoint
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // validate the message
    msg.validate()?;

    // save contract info
    let contract_info = ContractInfo::new(
        info.sender,
        msg.bind_name,
        msg.contract_name,
        CONTRACT_VERSION.into(),
    );
    set_contract_info(deps.storage, &contract_info)?;

    // build response
    Ok(Response {
        submessages: vec![],
        messages: vec![],
        attributes: vec![
            attr(
                "contract_info",
                format!("{:?}", get_contract_info(deps.storage)?),
            ),
            attr("action", "init"),
        ],
        data: None,
    })
}

// smart contract execute entrypoint
#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // validate the message
    msg.validate()?;

    match msg {
        // TODO: handle execute messages here
    }
}

// smart contract query entrypoint
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractInfo {} => to_binary(&get_contract_info(deps.storage)?),
        // TODO: handle query messages here
    }
}

// smart contract migrate/upgrade entrypoint
#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // always update version info
    let mut contract_info = get_contract_info(deps.storage)?;
    contract_info.version = CONTRACT_VERSION.into();
    set_contract_info(deps.storage, &contract_info)?;

    Ok(Response::default())
}
