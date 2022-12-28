use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/// Params defines the parameters for the oracle module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub vote_period: u64,
    #[prost(string, tag = "2")]
    pub vote_threshold: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub reward_band: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub reward_distribution_window: u64,
    #[prost(message, repeated, tag = "5")]
    pub accept_list: ::prost::alloc::vec::Vec<Denom>,
    #[prost(string, tag = "6")]
    pub slash_fraction: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub slash_window: u64,
    #[prost(string, tag = "8")]
    pub min_valid_per_window: ::prost::alloc::string::String,
}
/// Denom - the object to hold configurations of each denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Denom {
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol_denom: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub exponent: u32,
}
/// AggregateExchangeRatePrevote -
/// struct for aggregate prevoting on the ExchangeRateVote.
/// The purpose of aggregate prevote is to hide vote exchange rates with hash
/// which is formatted as hex string in SHA256("{salt}:{exchange
/// rate}{denom},...,{exchange rate}{denom}:{voter}")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub submit_block: u64,
}
/// AggregateExchangeRateVote - struct for voting on
/// the exchange rates of USD denominated in various assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRateVote {
    #[prost(message, repeated, tag = "1")]
    pub exchange_rate_tuples: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// ExchangeRateTuple - struct to store interpreted exchange rates data to store
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateTuple {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRatePrevote represents a message to submit an aggregate
/// exchange rate prevote.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevote {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// Feeder is the author and the signer of the message.
    #[prost(string, tag = "2")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRatePrevoteResponse defines the
/// Msg/AggregateExchangeRatePrevote response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevoteResponse {}
/// MsgAggregateExchangeRateVote represents a message to submit anaggregate
/// exchange rate vote.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVote {
    #[prost(string, tag = "1")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exchange_rates: ::prost::alloc::string::String,
    /// Feeder is the author and the signer of the message.
    #[prost(string, tag = "3")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRateVoteResponse defines the
/// Msg/AggregateExchangeRateVote response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVoteResponse {}
/// MsgDelegateFeedConsent represents a message to delegate oracle voting rights
/// to another address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsent {
    /// Operator is the author and the signer of the message.
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub delegate: ::prost::alloc::string::String,
}
/// MsgDelegateFeedConsentResponse defines the Msg/DelegateFeedConsent response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsentResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the oracle Msg service.
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
        /// AggregateExchangeRatePrevote defines a method for submitting an aggregate
        /// exchange rate prevote.
        pub async fn aggregate_exchange_rate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRatePrevote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/juno.oracle.v1.Msg/AggregateExchangeRatePrevote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateExchangeRateVote defines a method for submitting an aggregate
        /// exchange rate vote.
        pub async fn aggregate_exchange_rate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRateVote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRateVoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/juno.oracle.v1.Msg/AggregateExchangeRateVote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegateFeedConsent defines a method for setting the feeder delegation.
        pub async fn delegate_feed_consent(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelegateFeedConsent>,
        ) -> Result<tonic::Response<super::MsgDelegateFeedConsentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/juno.oracle.v1.Msg/DelegateFeedConsent");
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
        /// AggregateExchangeRatePrevote defines a method for submitting an aggregate
        /// exchange rate prevote.
        async fn aggregate_exchange_rate_prevote(
            &self,
            request: tonic::Request<super::MsgAggregateExchangeRatePrevote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>, tonic::Status>;
        /// AggregateExchangeRateVote defines a method for submitting an aggregate
        /// exchange rate vote.
        async fn aggregate_exchange_rate_vote(
            &self,
            request: tonic::Request<super::MsgAggregateExchangeRateVote>,
        ) -> Result<tonic::Response<super::MsgAggregateExchangeRateVoteResponse>, tonic::Status>;
        /// DelegateFeedConsent defines a method for setting the feeder delegation.
        async fn delegate_feed_consent(
            &self,
            request: tonic::Request<super::MsgDelegateFeedConsent>,
        ) -> Result<tonic::Response<super::MsgDelegateFeedConsentResponse>, tonic::Status>;
    }
    /// Msg defines the oracle Msg service.
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
                "/juno.oracle.v1.Msg/AggregateExchangeRatePrevote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateExchangeRatePrevoteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAggregateExchangeRatePrevote>
                        for AggregateExchangeRatePrevoteSvc<T>
                    {
                        type Response = super::MsgAggregateExchangeRatePrevoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAggregateExchangeRatePrevote>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).aggregate_exchange_rate_prevote(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregateExchangeRatePrevoteSvc(inner);
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
                "/juno.oracle.v1.Msg/AggregateExchangeRateVote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateExchangeRateVoteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAggregateExchangeRateVote>
                        for AggregateExchangeRateVoteSvc<T>
                    {
                        type Response = super::MsgAggregateExchangeRateVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAggregateExchangeRateVote>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).aggregate_exchange_rate_vote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregateExchangeRateVoteSvc(inner);
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
                "/juno.oracle.v1.Msg/DelegateFeedConsent" => {
                    #[allow(non_camel_case_types)]
                    struct DelegateFeedConsentSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDelegateFeedConsent>
                        for DelegateFeedConsentSvc<T>
                    {
                        type Response = super::MsgDelegateFeedConsentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDelegateFeedConsent>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegate_feed_consent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegateFeedConsentSvc(inner);
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
        const NAME: &'static str = "juno.oracle.v1.Msg";
    }
}
/// QueryExchangeRates is the request type for the Query/ExchangeRate RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRates {
    /// denom defines the denomination to query for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryExchangeRatesResponse is response type for the
/// Query/ExchangeRates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRatesResponse {
    /// exchange_rates defines a list of the exchange rate for all whitelisted
    /// denoms.
    #[prost(message, repeated, tag = "1")]
    pub exchange_rates:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryActiveExchangeRates is the request type for the
/// Query/ActiveExchangeRates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveExchangeRates {}
/// QueryActiveExchangeRatesResponse is response type for the
/// Query/ActiveExchangeRates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveExchangeRatesResponse {
    /// activeRates defines a list of the denomination which oracle prices aggreed
    /// upon.
    #[prost(string, repeated, tag = "1")]
    pub active_rates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeederDelegation is the request type for the
/// Query/FeederDelegation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegation {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryFeederDelegationResponse is response type for the
/// Query/FeederDelegation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegationResponse {
    /// feeder_addr defines the feeder delegation of a validator
    #[prost(string, tag = "1")]
    pub feeder_addr: ::prost::alloc::string::String,
}
/// QueryMissCounter is the request type for the Query/MissCounter RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounter {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterResponse is response type for the
/// Query/MissCounter RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounterResponse {
    /// miss_counter defines the oracle miss counter of a validator
    #[prost(uint64, tag = "1")]
    pub miss_counter: u64,
}
/// QuerySlashWindow is the request type for the
/// Query/SlashWindow RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySlashWindow {}
/// QuerySlashWindowResponse is response type for the
/// Query/SlashWindow RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySlashWindowResponse {
    /// window_progress defines the number of voting periods
    /// since the last slashing event would have taken place.
    #[prost(uint64, tag = "1")]
    pub window_progress: u64,
}
/// QueryAggregatePrevote is the request type for the
/// Query/AggregatePrevote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevote {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregatePrevoteResponse is response type for the
/// Query/AggregatePrevote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevoteResponse {
    /// aggregate_prevote defines oracle aggregate prevote submitted by a validator
    /// in the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_prevote: ::core::option::Option<AggregateExchangeRatePrevote>,
}
/// QueryAggregatePrevotes is the request type for the
/// Query/AggregatePrevotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotes {}
/// QueryAggregatePrevotesResponse is response type for the
/// Query/AggregatePrevotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotesResponse {
    /// aggregate_prevotes defines all oracle aggregate prevotes submitted in the
    /// current vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
}
/// QueryAggregateVote is the request type for the Query/AggregateVote RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVote {
    /// validator defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregateVoteResponse is response type for the
/// Query/AggregateVote RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVoteResponse {
    /// aggregate_vote defines oracle aggregate vote submitted by a validator in
    /// the current vote period
    #[prost(message, optional, tag = "1")]
    pub aggregate_vote: ::core::option::Option<AggregateExchangeRateVote>,
}
/// QueryAggregateVotes is the request type for the Query/AggregateVotes
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotes {}
/// QueryAggregateVotesResponse is response type for the
/// Query/AggregateVotes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotesResponse {
    /// aggregate_votes defines all oracle aggregate votes submitted in the current
    /// vote period
    #[prost(message, repeated, tag = "1")]
    pub aggregate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
}
/// QueryParams is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParams {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
        /// ExchangeRates returns exchange rates of all denoms,
        /// or, if specified, returns a single denom
        pub async fn exchange_rates(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRates>,
        ) -> Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/ExchangeRates");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ActiveExchangeRates returns all active denoms
        pub async fn active_exchange_rates(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryActiveExchangeRates>,
        ) -> Result<tonic::Response<super::QueryActiveExchangeRatesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/ActiveExchangeRates");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// FeederDelegation returns feeder delegation of a validator
        pub async fn feeder_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeederDelegation>,
        ) -> Result<tonic::Response<super::QueryFeederDelegationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/FeederDelegation");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MissCounter returns oracle miss counter of a validator
        pub async fn miss_counter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMissCounter>,
        ) -> Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/MissCounter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SlashWindow returns slash window information
        pub async fn slash_window(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySlashWindow>,
        ) -> Result<tonic::Response<super::QuerySlashWindowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/SlashWindow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregatePrevote returns an aggregate prevote of a validator
        pub async fn aggregate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevote>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/AggregatePrevote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregatePrevotes returns aggregate prevotes of all validators
        pub async fn aggregate_prevotes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevotes>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/AggregatePrevotes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateVote returns an aggregate vote of a validator
        pub async fn aggregate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVote>,
        ) -> Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/AggregateVote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateVotes returns aggregate votes of all validators
        pub async fn aggregate_votes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVotes>,
        ) -> Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/AggregateVotes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Params queries all parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParams>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/juno.oracle.v1.Query/Params");
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
        /// ExchangeRates returns exchange rates of all denoms,
        /// or, if specified, returns a single denom
        async fn exchange_rates(
            &self,
            request: tonic::Request<super::QueryExchangeRates>,
        ) -> Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status>;
        /// ActiveExchangeRates returns all active denoms
        async fn active_exchange_rates(
            &self,
            request: tonic::Request<super::QueryActiveExchangeRates>,
        ) -> Result<tonic::Response<super::QueryActiveExchangeRatesResponse>, tonic::Status>;
        /// FeederDelegation returns feeder delegation of a validator
        async fn feeder_delegation(
            &self,
            request: tonic::Request<super::QueryFeederDelegation>,
        ) -> Result<tonic::Response<super::QueryFeederDelegationResponse>, tonic::Status>;
        /// MissCounter returns oracle miss counter of a validator
        async fn miss_counter(
            &self,
            request: tonic::Request<super::QueryMissCounter>,
        ) -> Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status>;
        /// SlashWindow returns slash window information
        async fn slash_window(
            &self,
            request: tonic::Request<super::QuerySlashWindow>,
        ) -> Result<tonic::Response<super::QuerySlashWindowResponse>, tonic::Status>;
        /// AggregatePrevote returns an aggregate prevote of a validator
        async fn aggregate_prevote(
            &self,
            request: tonic::Request<super::QueryAggregatePrevote>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevoteResponse>, tonic::Status>;
        /// AggregatePrevotes returns aggregate prevotes of all validators
        async fn aggregate_prevotes(
            &self,
            request: tonic::Request<super::QueryAggregatePrevotes>,
        ) -> Result<tonic::Response<super::QueryAggregatePrevotesResponse>, tonic::Status>;
        /// AggregateVote returns an aggregate vote of a validator
        async fn aggregate_vote(
            &self,
            request: tonic::Request<super::QueryAggregateVote>,
        ) -> Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status>;
        /// AggregateVotes returns aggregate votes of all validators
        async fn aggregate_votes(
            &self,
            request: tonic::Request<super::QueryAggregateVotes>,
        ) -> Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status>;
        /// Params queries all parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParams>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
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
                "/juno.oracle.v1.Query/ExchangeRates" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeRatesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryExchangeRates> for ExchangeRatesSvc<T> {
                        type Response = super::QueryExchangeRatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryExchangeRates>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exchange_rates(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangeRatesSvc(inner);
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
                "/juno.oracle.v1.Query/ActiveExchangeRates" => {
                    #[allow(non_camel_case_types)]
                    struct ActiveExchangeRatesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryActiveExchangeRates>
                        for ActiveExchangeRatesSvc<T>
                    {
                        type Response = super::QueryActiveExchangeRatesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryActiveExchangeRates>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).active_exchange_rates(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ActiveExchangeRatesSvc(inner);
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
                "/juno.oracle.v1.Query/FeederDelegation" => {
                    #[allow(non_camel_case_types)]
                    struct FeederDelegationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFeederDelegation>
                        for FeederDelegationSvc<T>
                    {
                        type Response = super::QueryFeederDelegationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFeederDelegation>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).feeder_delegation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FeederDelegationSvc(inner);
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
                "/juno.oracle.v1.Query/MissCounter" => {
                    #[allow(non_camel_case_types)]
                    struct MissCounterSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryMissCounter> for MissCounterSvc<T> {
                        type Response = super::QueryMissCounterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMissCounter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).miss_counter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MissCounterSvc(inner);
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
                "/juno.oracle.v1.Query/SlashWindow" => {
                    #[allow(non_camel_case_types)]
                    struct SlashWindowSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySlashWindow> for SlashWindowSvc<T> {
                        type Response = super::QuerySlashWindowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySlashWindow>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).slash_window(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SlashWindowSvc(inner);
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
                "/juno.oracle.v1.Query/AggregatePrevote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregatePrevoteSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregatePrevote>
                        for AggregatePrevoteSvc<T>
                    {
                        type Response = super::QueryAggregatePrevoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregatePrevote>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).aggregate_prevote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregatePrevoteSvc(inner);
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
                "/juno.oracle.v1.Query/AggregatePrevotes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregatePrevotesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregatePrevotes>
                        for AggregatePrevotesSvc<T>
                    {
                        type Response = super::QueryAggregatePrevotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregatePrevotes>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).aggregate_prevotes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregatePrevotesSvc(inner);
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
                "/juno.oracle.v1.Query/AggregateVote" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVoteSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregateVote> for AggregateVoteSvc<T> {
                        type Response = super::QueryAggregateVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVote>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).aggregate_vote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregateVoteSvc(inner);
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
                "/juno.oracle.v1.Query/AggregateVotes" => {
                    #[allow(non_camel_case_types)]
                    struct AggregateVotesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAggregateVotes> for AggregateVotesSvc<T> {
                        type Response = super::QueryAggregateVotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAggregateVotes>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).aggregate_votes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AggregateVotesSvc(inner);
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
                "/juno.oracle.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParams> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParams>,
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
        const NAME: &'static str = "juno.oracle.v1.Query";
    }
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub feeder_delegations: ::prost::alloc::vec::Vec<FeederDelegation>,
    #[prost(message, repeated, tag = "3")]
    pub exchange_rates: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(message, repeated, tag = "4")]
    pub miss_counters: ::prost::alloc::vec::Vec<MissCounter>,
    #[prost(message, repeated, tag = "5")]
    pub aggregate_exchange_rate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
    #[prost(message, repeated, tag = "6")]
    pub aggregate_exchange_rate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
}
/// FeederDelegation is the address for where oracle feeder authority are
/// delegated to. By default this struct is only used at genesis to feed in
/// default feeder addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeederDelegation {
    #[prost(string, tag = "1")]
    pub feeder_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MissCounter defines an miss counter and validator address pair used in
/// oracle module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissCounter {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub miss_counter: u64,
}
/// EventDelegateFeedConsent is emitted on Msg/DelegateFeedConsent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDelegateFeedConsent {
    /// Operator bech32 address who delegates his feed consent
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    /// Delegate bech32 address
    #[prost(string, tag = "2")]
    pub delegate: ::prost::alloc::string::String,
}
/// EventSetFxRate is emitted on exchange rate update
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetFxRate {
    /// uToken denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Exchange rate (based to USD)
    #[prost(string, tag = "2")]
    pub rate: ::prost::alloc::string::String,
}
