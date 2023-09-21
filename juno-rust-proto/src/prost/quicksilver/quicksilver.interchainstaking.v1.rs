#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zone {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub deposit_address: ::core::option::Option<IcaAccount>,
    #[prost(message, optional, tag = "4")]
    pub withdrawal_address: ::core::option::Option<IcaAccount>,
    #[prost(message, optional, tag = "5")]
    pub performance_address: ::core::option::Option<IcaAccount>,
    #[prost(message, optional, tag = "6")]
    pub delegation_address: ::core::option::Option<IcaAccount>,
    #[prost(string, tag = "7")]
    pub account_prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub local_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub redemption_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub last_redemption_rate: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "12")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, repeated, tag = "13")]
    pub aggregate_intent: ::prost::alloc::vec::Vec<ValidatorIntent>,
    /// deprecated
    #[prost(bool, tag = "14")]
    pub multi_send: bool,
    #[prost(bool, tag = "15")]
    pub liquidity_module: bool,
    #[prost(uint32, tag = "16")]
    pub withdrawal_waitgroup: u32,
    #[prost(bytes = "vec", tag = "17")]
    pub ibc_next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "18")]
    pub validator_selection_allocation: u64,
    #[prost(uint64, tag = "19")]
    pub holdings_allocation: u64,
    /// deprecated
    #[prost(int64, tag = "20")]
    pub last_epoch_height: i64,
    #[prost(string, tag = "21")]
    pub tvl: ::prost::alloc::string::String,
    #[prost(int64, tag = "22")]
    pub unbonding_period: i64,
    #[prost(int64, tag = "23")]
    pub messages_per_tx: i64,
    #[prost(int64, tag = "24")]
    pub decimals: i64,
    #[prost(bool, tag = "25")]
    pub unbonding_enabled: bool,
    #[prost(bool, tag = "26")]
    pub deposits_enabled: bool,
    #[prost(bool, tag = "27")]
    pub return_to_sender: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IcaAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// balance defines the different coins this balance holds.
    #[prost(message, repeated, tag = "2")]
    pub balance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub port_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub withdrawal_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub balance_waitgroup: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    #[prost(string, tag = "1")]
    pub valoper: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawalRecord {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub distribution: ::prost::alloc::vec::Vec<Distribution>,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub burn_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "7")]
    pub txhash: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub status: i32,
    #[prost(message, optional, tag = "9")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "10")]
    pub requeued: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnbondingRecord {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub epoch_number: i64,
    #[prost(string, tag = "3")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub related_txhash: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedelegationRecord {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub epoch_number: i64,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub destination: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub amount: i64,
    #[prost(message, optional, tag = "6")]
    pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRecord {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub valoper_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub commission_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub delegator_shares: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub voting_power: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub score: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub status: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub jailed: bool,
    #[prost(bool, tag = "8")]
    pub tombstoned: bool,
    #[prost(message, optional, tag = "9")]
    pub jailed_since: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorIntent {
    #[prost(string, tag = "1")]
    pub delegator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub intents: ::prost::alloc::vec::Vec<ValidatorIntent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorIntent {
    #[prost(string, tag = "1")]
    pub valoper_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delegation {
    #[prost(string, tag = "1")]
    pub delegation_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "4")]
    pub height: i64,
    #[prost(int64, tag = "5")]
    pub redelegation_end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortConnectionTuple {
    #[prost(string, tag = "1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub txhash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub first_seen: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub completed: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statistics {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub deposited: i64,
    #[prost(int64, tag = "3")]
    pub deposits: i64,
    #[prost(int64, tag = "4")]
    pub depositors: i64,
    #[prost(int64, tag = "5")]
    pub delegated: i64,
    #[prost(int64, tag = "6")]
    pub supply: i64,
    #[prost(string, tag = "7")]
    pub distance_to_target: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZonesRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZonesResponse {
    #[prost(message, repeated, tag = "1")]
    pub zones: ::prost::alloc::vec::Vec<Zone>,
    #[prost(message, repeated, tag = "2")]
    pub stats: ::prost::alloc::vec::Vec<Statistics>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneResponse {
    #[prost(message, optional, tag = "1")]
    pub zone: ::core::option::Option<Zone>,
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<Statistics>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneValidatorsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneValidatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryDepositAccountForChainRequest is the request type for the
/// Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositAccountForChainRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// QueryDepositAccountForChainResponse the response type for the
/// Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDepositAccountForChainResponse {
    #[prost(string, tag = "1")]
    pub deposit_account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorIntentRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorIntentResponse {
    #[prost(message, optional, tag = "1")]
    pub intent: ::core::option::Option<DelegatorIntent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub delegations: ::prost::alloc::vec::Vec<Delegation>,
    #[prost(int64, tag = "2")]
    pub tvl: i64,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryReceiptsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryReceiptsResponse {
    #[prost(message, repeated, tag = "1")]
    pub receipts: ::prost::alloc::vec::Vec<Receipt>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalRecordsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub withdrawals: ::prost::alloc::vec::Vec<WithdrawalRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingRecordsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnbondingRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbondings: ::prost::alloc::vec::Vec<UnbondingRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationRecordsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedelegationRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub redelegations: ::prost::alloc::vec::Vec<RedelegationRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Zones provides meta data on connected zones.
        pub async fn zones(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryZonesRequest>,
        ) -> Result<tonic::Response<super::QueryZonesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/Zones",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Zone provides meta data on a specific zone.
        pub async fn zone(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryZoneRequest>,
        ) -> Result<tonic::Response<super::QueryZoneResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/Zone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn zone_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryZoneValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryZoneValidatorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/ZoneValidators",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DepositAccount provides data on the deposit address for a connected zone.
        pub async fn deposit_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDepositAccountForChainRequest>,
        ) -> Result<tonic::Response<super::QueryDepositAccountForChainResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/DepositAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegatorIntent provides data on the intent of the delegator for the given
        /// zone.
        pub async fn delegator_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorIntentRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/DelegatorIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delegations provides data on the delegations for the given zone.
        pub async fn delegations(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/Delegations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delegations provides data on the delegations for the given zone.
        pub async fn receipts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryReceiptsRequest>,
        ) -> Result<tonic::Response<super::QueryReceiptsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/Receipts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WithdrawalRecords provides data on the active withdrawals.
        pub async fn zone_withdrawal_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawalRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawalRecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/ZoneWithdrawalRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WithdrawalRecords provides data on the active withdrawals.
        pub async fn withdrawal_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawalRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawalRecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/WithdrawalRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UnbondingRecords provides data on the active unbondings.
        pub async fn unbonding_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnbondingRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryUnbondingRecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/UnbondingRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RedelegationRecords provides data on the active unbondings.
        pub async fn redelegation_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRedelegationRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryRedelegationRecordsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Query/RedelegationRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Zones provides meta data on connected zones.
        async fn zones(
            &self,
            request: tonic::Request<super::QueryZonesRequest>,
        ) -> Result<tonic::Response<super::QueryZonesResponse>, tonic::Status>;
        /// Zone provides meta data on a specific zone.
        async fn zone(
            &self,
            request: tonic::Request<super::QueryZoneRequest>,
        ) -> Result<tonic::Response<super::QueryZoneResponse>, tonic::Status>;
        async fn zone_validators(
            &self,
            request: tonic::Request<super::QueryZoneValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryZoneValidatorsResponse>, tonic::Status>;
        /// DepositAccount provides data on the deposit address for a connected zone.
        async fn deposit_account(
            &self,
            request: tonic::Request<super::QueryDepositAccountForChainRequest>,
        ) -> Result<tonic::Response<super::QueryDepositAccountForChainResponse>, tonic::Status>;
        /// DelegatorIntent provides data on the intent of the delegator for the given
        /// zone.
        async fn delegator_intent(
            &self,
            request: tonic::Request<super::QueryDelegatorIntentRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorIntentResponse>, tonic::Status>;
        /// Delegations provides data on the delegations for the given zone.
        async fn delegations(
            &self,
            request: tonic::Request<super::QueryDelegationsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationsResponse>, tonic::Status>;
        /// Delegations provides data on the delegations for the given zone.
        async fn receipts(
            &self,
            request: tonic::Request<super::QueryReceiptsRequest>,
        ) -> Result<tonic::Response<super::QueryReceiptsResponse>, tonic::Status>;
        /// WithdrawalRecords provides data on the active withdrawals.
        async fn zone_withdrawal_records(
            &self,
            request: tonic::Request<super::QueryWithdrawalRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawalRecordsResponse>, tonic::Status>;
        /// WithdrawalRecords provides data on the active withdrawals.
        async fn withdrawal_records(
            &self,
            request: tonic::Request<super::QueryWithdrawalRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryWithdrawalRecordsResponse>, tonic::Status>;
        /// UnbondingRecords provides data on the active unbondings.
        async fn unbonding_records(
            &self,
            request: tonic::Request<super::QueryUnbondingRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryUnbondingRecordsResponse>, tonic::Status>;
        /// RedelegationRecords provides data on the active unbondings.
        async fn redelegation_records(
            &self,
            request: tonic::Request<super::QueryRedelegationRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryRedelegationRecordsResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/quicksilver.interchainstaking.v1.Query/Zones" => {
                    #[allow(non_camel_case_types)]
                    struct ZonesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryZonesRequest> for ZonesSvc<T> {
                        type Response = super::QueryZonesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryZonesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).zones(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZonesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/Zone" => {
                    #[allow(non_camel_case_types)]
                    struct ZoneSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryZoneRequest> for ZoneSvc<T> {
                        type Response = super::QueryZoneResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryZoneRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).zone(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZoneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/ZoneValidators" => {
                    #[allow(non_camel_case_types)]
                    struct ZoneValidatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryZoneValidatorsRequest>
                        for ZoneValidatorsSvc<T>
                    {
                        type Response = super::QueryZoneValidatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryZoneValidatorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).zone_validators(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZoneValidatorsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/DepositAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DepositAccountSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDepositAccountForChainRequest>
                        for DepositAccountSvc<T>
                    {
                        type Response = super::QueryDepositAccountForChainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDepositAccountForChainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deposit_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DepositAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/DelegatorIntent" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorIntentSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDelegatorIntentRequest>
                        for DelegatorIntentSvc<T>
                    {
                        type Response = super::QueryDelegatorIntentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegatorIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegator_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegatorIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/Delegations" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDelegationsRequest> for DelegationsSvc<T> {
                        type Response = super::QueryDelegationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/Receipts" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiptsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryReceiptsRequest> for ReceiptsSvc<T> {
                        type Response = super::QueryReceiptsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryReceiptsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).receipts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReceiptsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/ZoneWithdrawalRecords" => {
                    #[allow(non_camel_case_types)]
                    struct ZoneWithdrawalRecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryWithdrawalRecordsRequest>
                        for ZoneWithdrawalRecordsSvc<T>
                    {
                        type Response = super::QueryWithdrawalRecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawalRecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).zone_withdrawal_records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZoneWithdrawalRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/WithdrawalRecords" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawalRecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryWithdrawalRecordsRequest>
                        for WithdrawalRecordsSvc<T>
                    {
                        type Response = super::QueryWithdrawalRecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawalRecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).withdrawal_records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawalRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/UnbondingRecords" => {
                    #[allow(non_camel_case_types)]
                    struct UnbondingRecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryUnbondingRecordsRequest>
                        for UnbondingRecordsSvc<T>
                    {
                        type Response = super::QueryUnbondingRecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUnbondingRecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unbonding_records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnbondingRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Query/RedelegationRecords" => {
                    #[allow(non_camel_case_types)]
                    struct RedelegationRecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRedelegationRecordsRequest>
                        for RedelegationRecordsSvc<T>
                    {
                        type Response = super::QueryRedelegationRecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRedelegationRecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).redelegation_records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RedelegationRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "quicksilver.interchainstaking.v1.Query";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterZoneProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub local_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub account_prefix: ::prost::alloc::string::String,
    /// deprecated
    #[prost(bool, tag = "7")]
    pub multi_send: bool,
    #[prost(bool, tag = "8")]
    pub liquidity_module: bool,
    #[prost(int64, tag = "9")]
    pub messages_per_tx: i64,
    #[prost(bool, tag = "10")]
    pub return_to_sender: bool,
    #[prost(bool, tag = "11")]
    pub deposits_enabled: bool,
    #[prost(bool, tag = "12")]
    pub unbonding_enabled: bool,
    #[prost(int64, tag = "13")]
    pub decimals: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterZoneProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub local_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub account_prefix: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub multi_send: bool,
    #[prost(bool, tag = "8")]
    pub liquidity_module: bool,
    #[prost(string, tag = "9")]
    pub deposit: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub messages_per_tx: i64,
    #[prost(bool, tag = "11")]
    pub return_to_sender: bool,
    #[prost(bool, tag = "12")]
    pub deposits_enabled: bool,
    #[prost(bool, tag = "13")]
    pub unbonding_enabled: bool,
    #[prost(int64, tag = "14")]
    pub decimals: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateZoneProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<UpdateZoneValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateZoneProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<UpdateZoneValue>,
    #[prost(string, tag = "5")]
    pub deposit: ::prost::alloc::string::String,
}
/// UpdateZoneValue defines an individual parameter change, for use in
/// UpdateZoneProposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateZoneValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovReopenChannel {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgGovReopenChannelResponse defines the MsgGovReopenChannel response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovReopenChannelResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovCloseChannel {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgGovCloseChannelResponse defines the MsgGovCloseChannel response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovCloseChannelResponse {}
/// MsgRequestRedemption represents a message type to request a burn of qAssets
/// for native assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestRedemption {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub destination_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgSignalIntent represents a message type for signalling voting intent for
/// one or more validators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignalIntent {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub intents: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgRequestRedemptionResponse defines the MsgRequestRedemption response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestRedemptionResponse {}
/// MsgSignalIntentResponse defines the MsgSignalIntent response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignalIntentResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the interchainstaking Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// RequestRedemption defines a method for requesting burning of qAssets for
        /// native assets.
        pub async fn request_redemption(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRequestRedemption>,
        ) -> Result<tonic::Response<super::MsgRequestRedemptionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Msg/RequestRedemption",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SignalIntent defines a method for signalling voting intent for one or more
        /// validators.
        pub async fn signal_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSignalIntent>,
        ) -> Result<tonic::Response<super::MsgSignalIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Msg/SignalIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SignalIntent defines a method for signalling voting intent for one or more
        /// validators.
        pub async fn gov_close_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovCloseChannel>,
        ) -> Result<tonic::Response<super::MsgGovCloseChannelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Msg/GovCloseChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn gov_reopen_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovReopenChannel>,
        ) -> Result<tonic::Response<super::MsgGovReopenChannelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.interchainstaking.v1.Msg/GovReopenChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// RequestRedemption defines a method for requesting burning of qAssets for
        /// native assets.
        async fn request_redemption(
            &self,
            request: tonic::Request<super::MsgRequestRedemption>,
        ) -> Result<tonic::Response<super::MsgRequestRedemptionResponse>, tonic::Status>;
        /// SignalIntent defines a method for signalling voting intent for one or more
        /// validators.
        async fn signal_intent(
            &self,
            request: tonic::Request<super::MsgSignalIntent>,
        ) -> Result<tonic::Response<super::MsgSignalIntentResponse>, tonic::Status>;
        /// SignalIntent defines a method for signalling voting intent for one or more
        /// validators.
        async fn gov_close_channel(
            &self,
            request: tonic::Request<super::MsgGovCloseChannel>,
        ) -> Result<tonic::Response<super::MsgGovCloseChannelResponse>, tonic::Status>;
        async fn gov_reopen_channel(
            &self,
            request: tonic::Request<super::MsgGovReopenChannel>,
        ) -> Result<tonic::Response<super::MsgGovReopenChannelResponse>, tonic::Status>;
    }
    /// Msg defines the interchainstaking Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/quicksilver.interchainstaking.v1.Msg/RequestRedemption" => {
                    #[allow(non_camel_case_types)]
                    struct RequestRedemptionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRequestRedemption> for RequestRedemptionSvc<T> {
                        type Response = super::MsgRequestRedemptionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRequestRedemption>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).request_redemption(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RequestRedemptionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Msg/SignalIntent" => {
                    #[allow(non_camel_case_types)]
                    struct SignalIntentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSignalIntent> for SignalIntentSvc<T> {
                        type Response = super::MsgSignalIntentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSignalIntent>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).signal_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignalIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Msg/GovCloseChannel" => {
                    #[allow(non_camel_case_types)]
                    struct GovCloseChannelSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovCloseChannel> for GovCloseChannelSvc<T> {
                        type Response = super::MsgGovCloseChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovCloseChannel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).gov_close_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GovCloseChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quicksilver.interchainstaking.v1.Msg/GovReopenChannel" => {
                    #[allow(non_camel_case_types)]
                    struct GovReopenChannelSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovReopenChannel> for GovReopenChannelSvc<T> {
                        type Response = super::MsgGovReopenChannelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovReopenChannel>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).gov_reopen_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GovReopenChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "quicksilver.interchainstaking.v1.Msg";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsV1 {
    #[prost(uint64, tag = "1")]
    pub deposit_interval: u64,
    #[prost(uint64, tag = "2")]
    pub validatorset_interval: u64,
    #[prost(string, tag = "3")]
    pub commission_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub deposit_interval: u64,
    #[prost(uint64, tag = "2")]
    pub validatorset_interval: u64,
    #[prost(string, tag = "3")]
    pub commission_rate: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub unbonding_enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationsForZone {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub delegations: ::prost::alloc::vec::Vec<Delegation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorIntentsForZone {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub delegation_intent: ::prost::alloc::vec::Vec<DelegatorIntent>,
    #[prost(bool, tag = "3")]
    pub snapshot: bool,
}
/// GenesisState defines the interchainstaking module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub zones: ::prost::alloc::vec::Vec<Zone>,
    #[prost(message, repeated, tag = "3")]
    pub receipts: ::prost::alloc::vec::Vec<Receipt>,
    #[prost(message, repeated, tag = "4")]
    pub delegations: ::prost::alloc::vec::Vec<DelegationsForZone>,
    #[prost(message, repeated, tag = "5")]
    pub performance_delegations: ::prost::alloc::vec::Vec<DelegationsForZone>,
    #[prost(message, repeated, tag = "6")]
    pub delegator_intents: ::prost::alloc::vec::Vec<DelegatorIntentsForZone>,
    #[prost(message, repeated, tag = "7")]
    pub port_connections: ::prost::alloc::vec::Vec<PortConnectionTuple>,
    #[prost(message, repeated, tag = "8")]
    pub withdrawal_records: ::prost::alloc::vec::Vec<WithdrawalRecord>,
}
