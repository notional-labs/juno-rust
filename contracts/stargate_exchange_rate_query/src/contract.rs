use std::io::Cursor;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, SystemResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, msg::QueryMsg};
use base64::decode;
use bech32::encode;
use juno_rust_proto::juno::oracle::v1::{QueryExchangeRates, QueryParams};
use juno_rust_proto::osmosis::tokenfactory::v1beta1::{QueryParamsRequest, QueryParamsResponse};
use prost::{
    bytes::{Buf, Bytes},
    Message,
};

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
        }
        QueryMsg::QueryStargateParams {} => to_binary(&query_stargate_params(deps)?),
        QueryMsg::QueryHehe {} => to_binary(&query_hehe(deps)?),
    }
}

pub fn query_hehe(deps: Deps) -> StdResult<Binary> {
    Ok(to_binary("hehe")?)
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

    let res = match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => from_binary(&value),
    };
    res
}

pub fn query_stargate_params(deps: Deps) -> StdResult<Binary> {
    let query_request: QueryParamsRequest = QueryParamsRequest{};

    let vecu8_query_request = query_request.encode_to_vec();
    let data = Binary::from(vecu8_query_request);

    let query_request: QueryRequest<Empty> = QueryRequest::Stargate {
        path: "/osmosis.tokenfactory.v1beta1.Query/Params".to_string(),
        data: data,
    };

    let raw = to_vec(&query_request)
        .map_err(|serialize_err| {
            StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
        })
        .unwrap();
    match deps.querier.raw_query(&raw) {
        SystemResult::Err(system_err) => {
            return Err(StdError::generic_err(format!(
                "Querier contract error: {}",
                system_err
            )))
        }
        SystemResult::Ok(ContractResult::Err(contract_err)) => {
            return Err(StdError::generic_err(format!(
                "Querier contract error: {}",
                contract_err
            )))
        }
        SystemResult::Ok(ContractResult::Ok(value)) => {
            let mut a = decode(value.to_string()).map_err(|_| StdError::GenericErr {
                msg: "decode1".to_string(),
            })?;
            let mut b: QueryParamsResponse = QueryParamsResponse::default();
            QueryParamsResponse::encode(&mut b, &mut a).map_err(|_| StdError::GenericErr {
                msg: "decode2".to_string(),
            })?;
            // QueryParamsResponse::decode(a.).map_err(|_| StdError::GenericErr {
            //     msg: "decode3".to_string(),
            // })?;
            // QueryParamsResponse::encode(&b, &mut a).map_err(|_| StdError::GenericErr {
            //     msg: "decode2".to_string(),
            // })?;

            // QueryParamsResponse::decode(a).map_err(|_| StdError::GenericErr {
            //     msg: "decode3".to_string(),
            // })?;
            Ok(value)
        }
    }
}
