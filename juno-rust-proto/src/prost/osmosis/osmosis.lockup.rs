/// PeriodLock is a single unit of lock by period. It's a record of locked coin
/// at a specific time. It stores owner, duration, unlock time and the amount of
/// coins locked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodLock {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCondition {
    /// type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration = "LockQueryType", tag = "1")]
    pub lock_query_type: i32,
    /// What token denomination are we looking for lockups of
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// valid when query condition is ByDuration
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// valid when query condition is ByTime
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// SyntheticLock is a single unit of synthetic lockup
/// TODO: Change this to have
/// * underlying_lock_id
/// * synthetic_coin
/// * end_time
/// * duration
/// * owner
/// We then index synthetic locks by the denom, just like we do with normal
/// locks. Ideally we even get an interface, so we can re-use that same logic.
/// I currently have no idea how reward distribution is supposed to be working...
/// EVENTUALLY
/// we make a "constrained_coin" field, which is what the current "coins" field
/// is. Constrained coin field can be a #post-v7 feature, since we aren't
/// allowing partial unlocks of synthetic lockups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLock {
    /// underlying native lockup id for this synthetic lockup
    #[prost(uint64, tag = "1")]
    pub underlying_lock_id: u64,
    #[prost(string, tag = "2")]
    pub synth_denom: ::prost::alloc::string::String,
    /// used for unbonding synthetic lockups, for active synthetic lockups, this
    /// value is set to uninitialized value
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    /// Queries for locks that are longer than a certain duration
    ByDuration = 0,
    /// Queries for lockups that started before a specific time
    ByTime = 1,
}
impl LockQueryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LockQueryType::ByDuration => "ByDuration",
            LockQueryType::ByTime => "ByTime",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ByDuration" => Some(Self::ByDuration),
            "ByTime" => Some(Self::ByTime),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokens {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokensResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAll {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlocking {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// Amount of unlocking coins. Unlock all if not set.
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgExtendLockup extends the existing lockup's duration.
/// The new duration is longer than the original.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockup {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// duration to be set. fails if lower than the current duration, or is
    /// unlocking
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockupResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the Msg service.
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
        /// LockTokens lock tokens
        pub async fn lock_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLockTokens>,
        ) -> Result<tonic::Response<super::MsgLockTokensResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/LockTokens");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BeginUnlockingAll begin unlocking all tokens
        pub async fn begin_unlocking_all(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBeginUnlockingAll>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/BeginUnlockingAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MsgBeginUnlocking begins unlocking tokens by lock ID
        pub async fn begin_unlocking(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBeginUnlocking>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/BeginUnlocking");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MsgEditLockup edits the existing lockups by lock ID
        pub async fn extend_lockup(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExtendLockup>,
        ) -> Result<tonic::Response<super::MsgExtendLockupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Msg/ExtendLockup");
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
        /// LockTokens lock tokens
        async fn lock_tokens(
            &self,
            request: tonic::Request<super::MsgLockTokens>,
        ) -> Result<tonic::Response<super::MsgLockTokensResponse>, tonic::Status>;
        /// BeginUnlockingAll begin unlocking all tokens
        async fn begin_unlocking_all(
            &self,
            request: tonic::Request<super::MsgBeginUnlockingAll>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingAllResponse>, tonic::Status>;
        /// MsgBeginUnlocking begins unlocking tokens by lock ID
        async fn begin_unlocking(
            &self,
            request: tonic::Request<super::MsgBeginUnlocking>,
        ) -> Result<tonic::Response<super::MsgBeginUnlockingResponse>, tonic::Status>;
        /// MsgEditLockup edits the existing lockups by lock ID
        async fn extend_lockup(
            &self,
            request: tonic::Request<super::MsgExtendLockup>,
        ) -> Result<tonic::Response<super::MsgExtendLockupResponse>, tonic::Status>;
    }
    /// Msg defines the Msg service.
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
                "/osmosis.lockup.Msg/LockTokens" => {
                    #[allow(non_camel_case_types)]
                    struct LockTokensSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLockTokens> for LockTokensSvc<T> {
                        type Response = super::MsgLockTokensResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLockTokens>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lock_tokens(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockTokensSvc(inner);
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
                "/osmosis.lockup.Msg/BeginUnlockingAll" => {
                    #[allow(non_camel_case_types)]
                    struct BeginUnlockingAllSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBeginUnlockingAll> for BeginUnlockingAllSvc<T> {
                        type Response = super::MsgBeginUnlockingAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBeginUnlockingAll>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).begin_unlocking_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginUnlockingAllSvc(inner);
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
                "/osmosis.lockup.Msg/BeginUnlocking" => {
                    #[allow(non_camel_case_types)]
                    struct BeginUnlockingSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBeginUnlocking> for BeginUnlockingSvc<T> {
                        type Response = super::MsgBeginUnlockingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBeginUnlocking>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).begin_unlocking(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginUnlockingSvc(inner);
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
                "/osmosis.lockup.Msg/ExtendLockup" => {
                    #[allow(non_camel_case_types)]
                    struct ExtendLockupSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExtendLockup> for ExtendLockupSvc<T> {
                        type Response = super::MsgExtendLockupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExtendLockup>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).extend_lockup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExtendLockupSvc(inner);
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
        const NAME: &'static str = "osmosis.lockup.Msg";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedRequest {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedResponse {
    #[prost(message, optional, tag = "1")]
    pub lock: ::core::option::Option<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdRequest {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
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
        /// Return full balance of the module
        pub async fn module_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::ModuleBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/ModuleBalance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return locked balance of the module
        pub async fn module_locked_amount(
            &mut self,
            request: impl tonic::IntoRequest<super::ModuleLockedAmountRequest>,
        ) -> Result<tonic::Response<super::ModuleLockedAmountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/ModuleLockedAmount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns unlockable coins which are not withdrawn yet
        pub async fn account_unlockable_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockableCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockableCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountUnlockableCoins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns unlocking coins
        pub async fn account_unlocking_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockingCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockingCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountUnlockingCoins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return a locked coins that can't be withdrawn
        pub async fn account_locked_coins(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountLockedCoinsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedCoins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns locked records of an account with unlock time beyond timestamp
        pub async fn account_locked_past_time(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedPastTime");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns locked records of an account with unlock time beyond timestamp
        /// excluding tokens started unlocking
        pub async fn account_locked_past_time_not_unlocking_only(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedPastTimeNotUnlockingOnlyResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedPastTimeNotUnlockingOnly",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns unlocked records with unlock time before timestamp
        pub async fn account_unlocked_before_time(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUnlockedBeforeTimeRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockedBeforeTimeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountUnlockedBeforeTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns lock records by address, timestamp, denom
        pub async fn account_locked_past_time_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedPastTimeDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedPastTimeDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns total locked per denom with longer past given time
        pub async fn locked_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::LockedDenomRequest>,
        ) -> Result<tonic::Response<super::LockedDenomResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/LockedDenom");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns lock record by id
        pub async fn locked_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::LockedRequest>,
        ) -> Result<tonic::Response<super::LockedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/LockedByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns synthetic lockups by native lockup id
        pub async fn synthetic_lockups_by_lockup_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SyntheticLockupsByLockupIdRequest>,
        ) -> Result<tonic::Response<super::SyntheticLockupsByLockupIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/SyntheticLockupsByLockupID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns account locked records with longer duration
        pub async fn account_locked_longer_duration(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedLongerDuration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns account locked records with a specific duration
        pub async fn account_locked_duration(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedDurationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/osmosis.lockup.Query/AccountLockedDuration");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns account locked records with longer duration excluding tokens
        /// started unlocking
        pub async fn account_locked_longer_duration_not_unlocking_only(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedLongerDurationNotUnlockingOnlyResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedLongerDurationNotUnlockingOnly",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns account's locked records for a denom with longer duration
        pub async fn account_locked_longer_duration_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountLockedLongerDurationDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/osmosis.lockup.Query/AccountLockedLongerDurationDenom",
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
        /// Return full balance of the module
        async fn module_balance(
            &self,
            request: tonic::Request<super::ModuleBalanceRequest>,
        ) -> Result<tonic::Response<super::ModuleBalanceResponse>, tonic::Status>;
        /// Return locked balance of the module
        async fn module_locked_amount(
            &self,
            request: tonic::Request<super::ModuleLockedAmountRequest>,
        ) -> Result<tonic::Response<super::ModuleLockedAmountResponse>, tonic::Status>;
        /// Returns unlockable coins which are not withdrawn yet
        async fn account_unlockable_coins(
            &self,
            request: tonic::Request<super::AccountUnlockableCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockableCoinsResponse>, tonic::Status>;
        /// Returns unlocking coins
        async fn account_unlocking_coins(
            &self,
            request: tonic::Request<super::AccountUnlockingCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockingCoinsResponse>, tonic::Status>;
        /// Return a locked coins that can't be withdrawn
        async fn account_locked_coins(
            &self,
            request: tonic::Request<super::AccountLockedCoinsRequest>,
        ) -> Result<tonic::Response<super::AccountLockedCoinsResponse>, tonic::Status>;
        /// Returns locked records of an account with unlock time beyond timestamp
        async fn account_locked_past_time(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeResponse>, tonic::Status>;
        /// Returns locked records of an account with unlock time beyond timestamp
        /// excluding tokens started unlocking
        async fn account_locked_past_time_not_unlocking_only(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedPastTimeNotUnlockingOnlyResponse>,
            tonic::Status,
        >;
        /// Returns unlocked records with unlock time before timestamp
        async fn account_unlocked_before_time(
            &self,
            request: tonic::Request<super::AccountUnlockedBeforeTimeRequest>,
        ) -> Result<tonic::Response<super::AccountUnlockedBeforeTimeResponse>, tonic::Status>;
        /// Returns lock records by address, timestamp, denom
        async fn account_locked_past_time_denom(
            &self,
            request: tonic::Request<super::AccountLockedPastTimeDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedPastTimeDenomResponse>, tonic::Status>;
        /// Returns total locked per denom with longer past given time
        async fn locked_denom(
            &self,
            request: tonic::Request<super::LockedDenomRequest>,
        ) -> Result<tonic::Response<super::LockedDenomResponse>, tonic::Status>;
        /// Returns lock record by id
        async fn locked_by_id(
            &self,
            request: tonic::Request<super::LockedRequest>,
        ) -> Result<tonic::Response<super::LockedResponse>, tonic::Status>;
        /// Returns synthetic lockups by native lockup id
        async fn synthetic_lockups_by_lockup_id(
            &self,
            request: tonic::Request<super::SyntheticLockupsByLockupIdRequest>,
        ) -> Result<tonic::Response<super::SyntheticLockupsByLockupIdResponse>, tonic::Status>;
        /// Returns account locked records with longer duration
        async fn account_locked_longer_duration(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationResponse>, tonic::Status>;
        /// Returns account locked records with a specific duration
        async fn account_locked_duration(
            &self,
            request: tonic::Request<super::AccountLockedDurationRequest>,
        ) -> Result<tonic::Response<super::AccountLockedDurationResponse>, tonic::Status>;
        /// Returns account locked records with longer duration excluding tokens
        /// started unlocking
        async fn account_locked_longer_duration_not_unlocking_only(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationNotUnlockingOnlyRequest>,
        ) -> Result<
            tonic::Response<super::AccountLockedLongerDurationNotUnlockingOnlyResponse>,
            tonic::Status,
        >;
        /// Returns account's locked records for a denom with longer duration
        async fn account_locked_longer_duration_denom(
            &self,
            request: tonic::Request<super::AccountLockedLongerDurationDenomRequest>,
        ) -> Result<tonic::Response<super::AccountLockedLongerDurationDenomResponse>, tonic::Status>;
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
                "/osmosis.lockup.Query/ModuleBalance" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleBalanceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ModuleBalanceRequest> for ModuleBalanceSvc<T> {
                        type Response = super::ModuleBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModuleBalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).module_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModuleBalanceSvc(inner);
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
                "/osmosis.lockup.Query/ModuleLockedAmount" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleLockedAmountSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ModuleLockedAmountRequest>
                        for ModuleLockedAmountSvc<T>
                    {
                        type Response = super::ModuleLockedAmountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModuleLockedAmountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).module_locked_amount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModuleLockedAmountSvc(inner);
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
                "/osmosis.lockup.Query/AccountUnlockableCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockableCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountUnlockableCoinsRequest>
                        for AccountUnlockableCoinsSvc<T>
                    {
                        type Response = super::AccountUnlockableCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockableCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlockable_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockableCoinsSvc(inner);
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
                "/osmosis.lockup.Query/AccountUnlockingCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockingCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountUnlockingCoinsRequest>
                        for AccountUnlockingCoinsSvc<T>
                    {
                        type Response = super::AccountUnlockingCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockingCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlocking_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockingCoinsSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedCoins" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedCoinsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedCoinsRequest>
                        for AccountLockedCoinsSvc<T>
                    {
                        type Response = super::AccountLockedCoinsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedCoinsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).account_locked_coins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedCoinsSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedPastTime" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedPastTimeRequest>
                        for AccountLockedPastTimeSvc<T>
                    {
                        type Response = super::AccountLockedPastTimeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedPastTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_locked_past_time(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedPastTimeNotUnlockingOnly" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeNotUnlockingOnlySvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::AccountLockedPastTimeNotUnlockingOnlyRequest,
                        > for AccountLockedPastTimeNotUnlockingOnlySvc<T>
                    {
                        type Response = super::AccountLockedPastTimeNotUnlockingOnlyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AccountLockedPastTimeNotUnlockingOnlyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .account_locked_past_time_not_unlocking_only(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeNotUnlockingOnlySvc(inner);
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
                "/osmosis.lockup.Query/AccountUnlockedBeforeTime" => {
                    #[allow(non_camel_case_types)]
                    struct AccountUnlockedBeforeTimeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountUnlockedBeforeTimeRequest>
                        for AccountUnlockedBeforeTimeSvc<T>
                    {
                        type Response = super::AccountUnlockedBeforeTimeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUnlockedBeforeTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_unlocked_before_time(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountUnlockedBeforeTimeSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedPastTimeDenom" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedPastTimeDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedPastTimeDenomRequest>
                        for AccountLockedPastTimeDenomSvc<T>
                    {
                        type Response = super::AccountLockedPastTimeDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedPastTimeDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_past_time_denom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedPastTimeDenomSvc(inner);
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
                "/osmosis.lockup.Query/LockedDenom" => {
                    #[allow(non_camel_case_types)]
                    struct LockedDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::LockedDenomRequest> for LockedDenomSvc<T> {
                        type Response = super::LockedDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockedDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).locked_denom(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockedDenomSvc(inner);
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
                "/osmosis.lockup.Query/LockedByID" => {
                    #[allow(non_camel_case_types)]
                    struct LockedByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::LockedRequest> for LockedByIDSvc<T> {
                        type Response = super::LockedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).locked_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LockedByIDSvc(inner);
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
                "/osmosis.lockup.Query/SyntheticLockupsByLockupID" => {
                    #[allow(non_camel_case_types)]
                    struct SyntheticLockupsByLockupIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::SyntheticLockupsByLockupIdRequest>
                        for SyntheticLockupsByLockupIDSvc<T>
                    {
                        type Response = super::SyntheticLockupsByLockupIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyntheticLockupsByLockupIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).synthetic_lockups_by_lockup_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyntheticLockupsByLockupIDSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedLongerDuration" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedLongerDurationRequest>
                        for AccountLockedLongerDurationSvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedLongerDurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_longer_duration(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedDuration" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedDurationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::AccountLockedDurationRequest>
                        for AccountLockedDurationSvc<T>
                    {
                        type Response = super::AccountLockedDurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedDurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).account_locked_duration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedDurationSvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedLongerDurationNotUnlockingOnly" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationNotUnlockingOnlySvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::AccountLockedLongerDurationNotUnlockingOnlyRequest,
                        > for AccountLockedLongerDurationNotUnlockingOnlySvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationNotUnlockingOnlyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AccountLockedLongerDurationNotUnlockingOnlyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .account_locked_longer_duration_not_unlocking_only(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationNotUnlockingOnlySvc(inner);
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
                "/osmosis.lockup.Query/AccountLockedLongerDurationDenom" => {
                    #[allow(non_camel_case_types)]
                    struct AccountLockedLongerDurationDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::AccountLockedLongerDurationDenomRequest>
                        for AccountLockedLongerDurationDenomSvc<T>
                    {
                        type Response = super::AccountLockedLongerDurationDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountLockedLongerDurationDenomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).account_locked_longer_duration_denom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountLockedLongerDurationDenomSvc(inner);
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
        const NAME: &'static str = "osmosis.lockup.Query";
    }
}
/// GenesisState defines the lockup module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint64, tag = "1")]
    pub last_lock_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
    #[prost(message, repeated, tag = "3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
