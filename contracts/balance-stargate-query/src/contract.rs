#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, Empty,to_vec, SystemResult, ContractResult
};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use cosmos_sdk_proto::cosmos::bank::v1beta1::{QueryBalanceRequest};
use prost::{Message};
use crate::{error::ContractError, querier::QueryMsg};

use cw2::set_contract_version;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:active-pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NATIVE_JUNO_DENOM: &str = "ujuno";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryBalance { denom, address } => query_exchange_rate_stargate(deps, denom, address),
    }
}


pub fn query_exchange_rate_stargate(deps: Deps, denom: String, address: String) -> StdResult<Binary> {
    let query_request = QueryBalanceRequest {
        address,
        denom,
    };

    let vecu8_query_request = query_request.encode_to_vec();
    let data =Binary::from(vecu8_query_request);

    let query_request:QueryRequest<Empty> = QueryRequest::Stargate {
        path: "/cosmos.bank.v1beta1.Query/Balance".to_string(),
        data : data,
    };

    let raw = to_vec(&query_request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    }).unwrap();

    return match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    };

}