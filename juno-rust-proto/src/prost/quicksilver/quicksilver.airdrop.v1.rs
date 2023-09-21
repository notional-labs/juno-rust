/// ZoneDrop represents an airdrop for a specific zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneDrop {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "4")]
    pub decay: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint64, tag = "5")]
    pub allocation: u64,
    #[prost(string, repeated, tag = "6")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "7")]
    pub is_concluded: bool,
}
/// ClaimRecord represents a users' claim (including completed claims) for a
/// given zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimRecord {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// Protobuf3 does not allow enum as map key
    #[prost(map = "int32, message", tag = "3")]
    pub actions_completed: ::std::collections::HashMap<i32, CompletedAction>,
    #[prost(uint64, tag = "4")]
    pub max_allocation: u64,
    #[prost(uint64, tag = "5")]
    pub base_value: u64,
}
/// CompletedAction represents a claim action completed by the user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedAction {
    #[prost(message, optional, tag = "1")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "2")]
    pub claim_amount: u64,
}
/// Action is used as an enum to denote specific actions or tasks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// Undefined action (per protobuf spec)
    Undefined = 0,
    /// Initial claim action
    InitialClaim = 1,
    /// Deposit tier 1 (e.g. > 5% of base_value)
    DepositT1 = 2,
    /// Deposit tier 2 (e.g. > 10% of base_value)
    DepositT2 = 3,
    /// Deposit tier 3 (e.g. > 15% of base_value)
    DepositT3 = 4,
    /// Deposit tier 4 (e.g. > 22% of base_value)
    DepositT4 = 5,
    /// Deposit tier 5 (e.g. > 30% of base_value)
    DepositT5 = 6,
    /// Active QCK delegation
    StakeQck = 7,
    /// Intent is set
    SignalIntent = 8,
    /// Cast governance vote on QS
    QsGov = 9,
    /// Governance By Proxy (GbP): cast vote on remote zone
    GbP = 10,
    /// Provide liquidity on Osmosis
    Osmosis = 11,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Undefined => "ActionUndefined",
            Action::InitialClaim => "ActionInitialClaim",
            Action::DepositT1 => "ActionDepositT1",
            Action::DepositT2 => "ActionDepositT2",
            Action::DepositT3 => "ActionDepositT3",
            Action::DepositT4 => "ActionDepositT4",
            Action::DepositT5 => "ActionDepositT5",
            Action::StakeQck => "ActionStakeQCK",
            Action::SignalIntent => "ActionSignalIntent",
            Action::QsGov => "ActionQSGov",
            Action::GbP => "ActionGbP",
            Action::Osmosis => "ActionOsmosis",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ActionUndefined" => Some(Self::Undefined),
            "ActionInitialClaim" => Some(Self::InitialClaim),
            "ActionDepositT1" => Some(Self::DepositT1),
            "ActionDepositT2" => Some(Self::DepositT2),
            "ActionDepositT3" => Some(Self::DepositT3),
            "ActionDepositT4" => Some(Self::DepositT4),
            "ActionDepositT5" => Some(Self::DepositT5),
            "ActionStakeQCK" => Some(Self::StakeQck),
            "ActionSignalIntent" => Some(Self::SignalIntent),
            "ActionQSGov" => Some(Self::QsGov),
            "ActionGbP" => Some(Self::GbP),
            "ActionOsmosis" => Some(Self::Osmosis),
            _ => None,
        }
    }
}
/// Status is used as an enum to denote zone status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Undefined = 0,
    Active = 1,
    Future = 2,
    Expired = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Undefined => "StatusUndefined",
            Status::Active => "StatusActive",
            Status::Future => "StatusFuture",
            Status::Expired => "StatusExpired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "StatusUndefined" => Some(Self::Undefined),
            "StatusActive" => Some(Self::Active),
            "StatusFuture" => Some(Self::Future),
            "StatusExpired" => Some(Self::Expired),
            _ => None,
        }
    }
}
/// Params holds parameters for the airdrop module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryZoneDropRequest is the request type for Query/ZoneDrop RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneDropRequest {
    /// chain_id identifies the zone.
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// QueryZoneDropResponse is the response type for Query/ZoneDrop RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneDropResponse {
    #[prost(message, optional, tag = "1")]
    pub zone_drop: ::core::option::Option<ZoneDrop>,
}
/// QueryAccountBalanceRequest is the request type for Query/AccountBalance RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountBalanceRequest {
    /// chain_id identifies the zone.
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// QueryAccountBalanceResponse is the response type for Query/AccountBalance RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountBalanceResponse {
    #[prost(message, optional, tag = "1")]
    pub account_balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryZoneDropsRequest is the request type for Query/ZoneDrops RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneDropsRequest {
    /// status enables to query zone airdrops matching a given status:
    ///   - Active
    ///   - Future
    ///   - Expired
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryZoneDropResponse is the response type for Query/ZoneDrops RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryZoneDropsResponse {
    #[prost(message, repeated, tag = "1")]
    pub zone_drops: ::prost::alloc::vec::Vec<ZoneDrop>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClaimRecordRequest is the request type for Query/ClaimRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// QueryClaimRecordResponse is the response type for Query/ClaimRecord RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub claim_record: ::core::option::Option<ClaimRecord>,
}
/// QueryClaimRecordsRequest is the request type for Query/ClaimRecords RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordsRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryClaimRecordsResponse is the response type for Query/ClaimRecords RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub claim_records: ::prost::alloc::vec::Vec<ClaimRecord>,
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
    /// Query provides defines the gRPC querier service.
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
        /// Params returns the total set of airdrop parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ZoneDrop returns the details of the specified zone airdrop.
        pub async fn zone_drop(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryZoneDropRequest>,
        ) -> Result<tonic::Response<super::QueryZoneDropResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Query/ZoneDrop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AccountBalance returns the module account balance of the specified zone.
        pub async fn account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountBalanceRequest>,
        ) -> Result<tonic::Response<super::QueryAccountBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.airdrop.v1.Query/AccountBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ZoneDrops returns all zone airdrops of the specified status.
        pub async fn zone_drops(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryZoneDropsRequest>,
        ) -> Result<tonic::Response<super::QueryZoneDropsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Query/ZoneDrops");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimRecord returns the claim record that corresponds to the given zone and
        /// address.
        pub async fn claim_record(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimRecordRequest>,
        ) -> Result<tonic::Response<super::QueryClaimRecordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Query/ClaimRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimRecords returns all the claim records of the given zone.
        pub async fn claim_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryClaimRecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Query/ClaimRecords");
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
        /// Params returns the total set of airdrop parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// ZoneDrop returns the details of the specified zone airdrop.
        async fn zone_drop(
            &self,
            request: tonic::Request<super::QueryZoneDropRequest>,
        ) -> Result<tonic::Response<super::QueryZoneDropResponse>, tonic::Status>;
        /// AccountBalance returns the module account balance of the specified zone.
        async fn account_balance(
            &self,
            request: tonic::Request<super::QueryAccountBalanceRequest>,
        ) -> Result<tonic::Response<super::QueryAccountBalanceResponse>, tonic::Status>;
        /// ZoneDrops returns all zone airdrops of the specified status.
        async fn zone_drops(
            &self,
            request: tonic::Request<super::QueryZoneDropsRequest>,
        ) -> Result<tonic::Response<super::QueryZoneDropsResponse>, tonic::Status>;
        /// ClaimRecord returns the claim record that corresponds to the given zone and
        /// address.
        async fn claim_record(
            &self,
            request: tonic::Request<super::QueryClaimRecordRequest>,
        ) -> Result<tonic::Response<super::QueryClaimRecordResponse>, tonic::Status>;
        /// ClaimRecords returns all the claim records of the given zone.
        async fn claim_records(
            &self,
            request: tonic::Request<super::QueryClaimRecordsRequest>,
        ) -> Result<tonic::Response<super::QueryClaimRecordsResponse>, tonic::Status>;
    }
    /// Query provides defines the gRPC querier service.
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
                "/quicksilver.airdrop.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
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
                "/quicksilver.airdrop.v1.Query/ZoneDrop" => {
                    #[allow(non_camel_case_types)]
                    struct ZoneDropSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryZoneDropRequest> for ZoneDropSvc<T> {
                        type Response = super::QueryZoneDropResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryZoneDropRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).zone_drop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZoneDropSvc(inner);
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
                "/quicksilver.airdrop.v1.Query/AccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct AccountBalanceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAccountBalanceRequest>
                        for AccountBalanceSvc<T>
                    {
                        type Response = super::QueryAccountBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAccountBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).account_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountBalanceSvc(inner);
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
                "/quicksilver.airdrop.v1.Query/ZoneDrops" => {
                    #[allow(non_camel_case_types)]
                    struct ZoneDropsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryZoneDropsRequest> for ZoneDropsSvc<T> {
                        type Response = super::QueryZoneDropsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryZoneDropsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).zone_drops(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ZoneDropsSvc(inner);
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
                "/quicksilver.airdrop.v1.Query/ClaimRecord" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimRecordSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryClaimRecordRequest> for ClaimRecordSvc<T> {
                        type Response = super::QueryClaimRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClaimRecordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).claim_record(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimRecordSvc(inner);
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
                "/quicksilver.airdrop.v1.Query/ClaimRecords" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimRecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryClaimRecordsRequest> for ClaimRecordsSvc<T> {
                        type Response = super::QueryClaimRecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryClaimRecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).claim_records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimRecordsSvc(inner);
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
        const NAME: &'static str = "quicksilver.airdrop.v1.Query";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaim {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub action: i64,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub proofs: ::prost::alloc::vec::Vec<super::super::claimsmanager::v1::Proof>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimResponse {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// MsgIncentivePoolSpend represents a message to send coins from one account to another.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncentivePoolSpend {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgIncentivePoolSpendResponse defines the MsgIncentivePoolSpend response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncentivePoolSpendResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the airdrop Msg service.
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
        pub async fn claim(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgClaim>,
        ) -> Result<tonic::Response<super::MsgClaimResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/quicksilver.airdrop.v1.Msg/Claim");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn incentive_pool_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIncentivePoolSpend>,
        ) -> Result<tonic::Response<super::MsgIncentivePoolSpendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/quicksilver.airdrop.v1.Msg/IncentivePoolSpend",
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
        async fn claim(
            &self,
            request: tonic::Request<super::MsgClaim>,
        ) -> Result<tonic::Response<super::MsgClaimResponse>, tonic::Status>;
        async fn incentive_pool_spend(
            &self,
            request: tonic::Request<super::MsgIncentivePoolSpend>,
        ) -> Result<tonic::Response<super::MsgIncentivePoolSpendResponse>, tonic::Status>;
    }
    /// Msg defines the airdrop Msg service.
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
                "/quicksilver.airdrop.v1.Msg/Claim" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgClaim> for ClaimSvc<T> {
                        type Response = super::MsgClaimResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgClaim>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).claim(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimSvc(inner);
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
                "/quicksilver.airdrop.v1.Msg/IncentivePoolSpend" => {
                    #[allow(non_camel_case_types)]
                    struct IncentivePoolSpendSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgIncentivePoolSpend>
                        for IncentivePoolSpendSvc<T>
                    {
                        type Response = super::MsgIncentivePoolSpendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgIncentivePoolSpend>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).incentive_pool_spend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IncentivePoolSpendSvc(inner);
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
        const NAME: &'static str = "quicksilver.airdrop.v1.Msg";
    }
}
/// GenesisState defines the airdrop module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub zone_drops: ::prost::alloc::vec::Vec<ZoneDrop>,
    #[prost(message, repeated, tag = "3")]
    pub claim_records: ::prost::alloc::vec::Vec<ClaimRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterZoneDropProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub zone_drop: ::core::option::Option<ZoneDrop>,
    #[prost(bytes = "vec", tag = "4")]
    pub claim_records: ::prost::alloc::vec::Vec<u8>,
}
