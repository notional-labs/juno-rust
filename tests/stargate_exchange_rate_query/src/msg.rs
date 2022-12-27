use cosmwasm_std::Coin;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use juno_rust_proto::cosmos::base::v1beta1::DecCoin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryStargateExchangeRates { denom: String },
}

