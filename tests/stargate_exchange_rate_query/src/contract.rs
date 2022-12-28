#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo, QueryRequest, Response,
    StdError, StdResult, SystemResult, to_binary, from_binary,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, msg::QueryMsg};
use juno_rust_proto::juno::oracle::v1::{QueryExchangeRates, QueryParams};
use prost::Message;

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
        QueryMsg::QueryStargateExchangeRates { denom } => {
            to_binary(&query_stargate_exchange_rates(deps, denom)?)
        },
        QueryMsg::QueryStargateParams {  } => to_binary(&query_stargate_params(deps)?)
    }
}

pub fn query_stargate_exchange_rates(deps: Deps, denom: String) -> StdResult<Binary> {
    let query_request: QueryExchangeRates = QueryExchangeRates { denom: denom };

    let vecu8_query_request = query_request.encode_to_vec();
    let data = Binary::from(vecu8_query_request);

    let query_request: QueryRequest<Empty> = QueryRequest::Stargate {
        path: "/juno.oracle.v1.Query/ExchangeRates".to_string(),
        data: data,
    };

    let raw = to_vec(&query_request)
        .map_err(|serialize_err| {
            StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
        })
        .unwrap();

    let res =  match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => {
            from_binary(&value)
        }
    };
    res
}

pub fn query_stargate_params(deps: Deps) -> StdResult<Binary> {
    let query_request: QueryParams = QueryParams {};

    let vecu8_query_request = query_request.encode_to_vec();
    let data = Binary::from(vecu8_query_request);

    let query_request: QueryRequest<Empty> = QueryRequest::Stargate {
        path: "/juno.oracle.v1.Query/Params".to_string(),
        data: data,
    };

    let raw = to_vec(&query_request)
        .map_err(|serialize_err| {
            StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
        })
        .unwrap();

    let res =  match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => {
            from_binary(&value)
        }
    };
    res
}

