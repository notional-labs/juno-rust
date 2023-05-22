/// AccessTypeParam
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTypeParam {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub value: i32,
}
/// AccessConfig access control type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfig {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub permission: i32,
    /// Address
    /// Deprecated: replaced by addresses
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of wasm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub code_upload_access: ::core::option::Option<AccessConfig>,
    #[prost(enumeration = "AccessType", tag = "2")]
    pub instantiate_default_permission: i32,
}
/// CodeInfo is data for the uploaded contract WASM code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfo {
    /// CodeHash is the unique identifier created by wasmvm
    #[prost(bytes = "vec", tag = "1")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    /// Creator address who initially stored the code
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// InstantiateConfig access control to apply on contract creation, optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_config: ::core::option::Option<AccessConfig>,
}
/// ContractInfo stores a WASM contract instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    /// CodeID is the reference to the stored Wasm code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// Creator address who initially instantiated the contract
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Created Tx position when the contract was instantiated.
    /// This data should kept internal and not be exposed via query results. Just
    /// use for sorting
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(string, tag = "6")]
    pub ibc_port_id: ::prost::alloc::string::String,
    /// Extension is an extension point to store custom metadata within the
    /// persistence model.
    #[prost(message, optional, tag = "7")]
    pub extension: ::core::option::Option<::prost_types::Any>,
}
/// ContractCodeHistoryEntry metadata to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCodeHistoryEntry {
    #[prost(enumeration = "ContractCodeHistoryOperationType", tag = "1")]
    pub operation: i32,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "2")]
    pub code_id: u64,
    /// Updated Tx position when the operation was executed.
    #[prost(message, optional, tag = "3")]
    pub updated: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// AbsoluteTxPosition is a unique transaction position that allows for global
/// ordering of transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    /// TxIndex is a monotonic counter within the block (actual transaction index,
    /// or gas consumed)
    #[prost(uint64, tag = "2")]
    pub tx_index: u64,
}
/// Model is a struct that holds a KV pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// hex-encode key to read it better (this is often ascii)
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// base64-encode raw value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// AccessType permission types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessType {
    /// AccessTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// AccessTypeNobody forbidden
    Nobody = 1,
    /// AccessTypeOnlyAddress restricted to a single address
    /// Deprecated: use AccessTypeAnyOfAddresses instead
    OnlyAddress = 2,
    /// AccessTypeEverybody unrestricted
    Everybody = 3,
    /// AccessTypeAnyOfAddresses allow any of the addresses
    AnyOfAddresses = 4,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            AccessType::Nobody => "ACCESS_TYPE_NOBODY",
            AccessType::OnlyAddress => "ACCESS_TYPE_ONLY_ADDRESS",
            AccessType::Everybody => "ACCESS_TYPE_EVERYBODY",
            AccessType::AnyOfAddresses => "ACCESS_TYPE_ANY_OF_ADDRESSES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCESS_TYPE_NOBODY" => Some(Self::Nobody),
            "ACCESS_TYPE_ONLY_ADDRESS" => Some(Self::OnlyAddress),
            "ACCESS_TYPE_EVERYBODY" => Some(Self::Everybody),
            "ACCESS_TYPE_ANY_OF_ADDRESSES" => Some(Self::AnyOfAddresses),
            _ => None,
        }
    }
}
/// ContractCodeHistoryOperationType actions that caused a code change
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractCodeHistoryOperationType {
    /// ContractCodeHistoryOperationTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// ContractCodeHistoryOperationTypeInit on chain contract instantiation
    Init = 1,
    /// ContractCodeHistoryOperationTypeMigrate code migration
    Migrate = 2,
    /// ContractCodeHistoryOperationTypeGenesis based on genesis data
    Genesis = 3,
}
impl ContractCodeHistoryOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractCodeHistoryOperationType::Unspecified => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED"
            }
            ContractCodeHistoryOperationType::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            ContractCodeHistoryOperationType::Migrate => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
            }
            ContractCodeHistoryOperationType::Genesis => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => Some(Self::Init),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => Some(Self::Genesis),
            _ => None,
        }
    }
}
/// MsgStoreCode submit Wasm code to the system
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCode {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "2")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission access control to apply on contract creation,
    /// optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// MsgStoreCodeResponse returns store result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// Checksum is the sha256 hash of the stored code
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// MsgInstantiateContract create a new smart contract instance for the given
/// code id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "5")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgInstantiateContract2 create a new smart contract instance for the given
/// code id with a predicable address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContract2 {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "5")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Salt is an arbitrary value provided by the sender. Size can be 1 to 64.
    #[prost(bytes = "vec", tag = "7")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// FixMsg include the msg value into the hash for the predictable address.
    /// Default is false
    #[prost(bool, tag = "8")]
    pub fix_msg: bool,
}
/// MsgInstantiateContractResponse return instantiation result data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgInstantiateContract2Response return instantiation result data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContract2Response {
    /// Address is the bech32 address of the new contract instance.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgExecuteContract submits the given message data to a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on execution
    #[prost(message, repeated, tag = "5")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgExecuteContractResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContractResponse {
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContract runs a code upgrade/ downgrade for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "3")]
    pub code_id: u64,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MsgMigrateContractResponse returns contract migration result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMigrateContractResponse {
    /// Data contains same raw bytes returned as data from the wasm contract.
    /// (May be empty)
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateAdmin sets a new admin for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAdmin {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "2")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// MsgUpdateAdminResponse returns empty data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAdminResponse {}
/// MsgClearAdmin removes any admin stored for a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAdmin {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// MsgClearAdminResponse returns empty data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAdminResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the wasm Msg service.
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
        /// StoreCode to submit Wasm code to the system
        pub async fn store_code(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgStoreCode>,
        ) -> Result<tonic::Response<super::MsgStoreCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/StoreCode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///  InstantiateContract creates a new smart contract instance for the given
        ///  code id.
        pub async fn instantiate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantiateContract>,
        ) -> Result<tonic::Response<super::MsgInstantiateContractResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/InstantiateContract");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///  InstantiateContract2 creates a new smart contract instance for the given
        ///  code id with a predictable address
        pub async fn instantiate_contract2(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInstantiateContract2>,
        ) -> Result<tonic::Response<super::MsgInstantiateContract2Response>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/InstantiateContract2");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute submits the given message data to a smart contract
        pub async fn execute_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExecuteContract>,
        ) -> Result<tonic::Response<super::MsgExecuteContractResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/ExecuteContract");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Migrate runs a code upgrade/ downgrade for a smart contract
        pub async fn migrate_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMigrateContract>,
        ) -> Result<tonic::Response<super::MsgMigrateContractResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/MigrateContract");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateAdmin sets a new   admin for a smart contract
        pub async fn update_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAdmin>,
        ) -> Result<tonic::Response<super::MsgUpdateAdminResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/UpdateAdmin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClearAdmin removes any admin stored for a smart contract
        pub async fn clear_admin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgClearAdmin>,
        ) -> Result<tonic::Response<super::MsgClearAdminResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Msg/ClearAdmin");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState - genesis state of x/wasm
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<Code>,
    #[prost(message, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    #[prost(message, repeated, tag = "4")]
    pub sequences: ::prost::alloc::vec::Vec<Sequence>,
    #[prost(message, repeated, tag = "5")]
    pub gen_msgs: ::prost::alloc::vec::Vec<genesis_state::GenMsgs>,
}
/// Nested message and enum types in `GenesisState`.
pub mod genesis_state {
    /// GenMsgs define the messages that can be executed during genesis phase in
    /// order. The intention is to have more human readable data that is auditable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GenMsgs {
        /// sum is a single message
        #[prost(oneof = "gen_msgs::Sum", tags = "1, 2, 3")]
        pub sum: ::core::option::Option<gen_msgs::Sum>,
    }
    /// Nested message and enum types in `GenMsgs`.
    pub mod gen_msgs {
        /// sum is a single message
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Sum {
            #[prost(message, tag = "1")]
            StoreCode(super::super::MsgStoreCode),
            #[prost(message, tag = "2")]
            InstantiateContract(super::super::MsgInstantiateContract),
            /// MsgInstantiateContract2 intentionally not supported
            /// see <https://github.com/CosmWasm/wasmd/issues/987>
            #[prost(message, tag = "3")]
            ExecuteContract(super::super::MsgExecuteContract),
        }
    }
}
/// Code struct encompasses CodeInfo and CodeBytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Code {
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    #[prost(message, optional, tag = "2")]
    pub code_info: ::core::option::Option<CodeInfo>,
    #[prost(bytes = "vec", tag = "3")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Pinned to wasmvm cache
    #[prost(bool, tag = "4")]
    pub pinned: bool,
}
/// Contract struct encompasses ContractAddress, ContractInfo, and ContractState
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
    #[prost(message, repeated, tag = "3")]
    pub contract_state: ::prost::alloc::vec::Vec<Model>,
}
/// Sequence key and value of an id generation counter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sequence {
    #[prost(bytes = "vec", tag = "1")]
    pub id_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
/// QueryContractInfoRequest is the request type for the Query/ContractInfo RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractInfoRequest {
    /// address is the address of the contract to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryContractInfoResponse is the response type for the Query/ContractInfo RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractInfoResponse {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_info: ::core::option::Option<ContractInfo>,
}
/// QueryContractHistoryRequest is the request type for the Query/ContractHistory
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHistoryRequest {
    /// address is the address of the contract to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractHistoryResponse is the response type for the
/// Query/ContractHistory RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<ContractCodeHistoryEntry>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryContractsByCodeRequest is the request type for the Query/ContractsByCode
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractsByCodeResponse is the response type for the
/// Query/ContractsByCode RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCodeResponse {
    /// contracts are a set of contract addresses
    #[prost(string, repeated, tag = "1")]
    pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAllContractStateRequest is the request type for the
/// Query/AllContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllContractStateResponse is the response type for the
/// Query/AllContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllContractStateResponse {
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryRawContractStateRequest is the request type for the
/// Query/RawContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRawContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub query_data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryRawContractStateResponse is the response type for the
/// Query/RawContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRawContractStateResponse {
    /// Data contains the raw store data
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QuerySmartContractStateRequest is the request type for the
/// Query/SmartContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySmartContractStateRequest {
    /// address is the address of the contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// QueryData contains the query data passed to the contract
    #[prost(bytes = "vec", tag = "2")]
    pub query_data: ::prost::alloc::vec::Vec<u8>,
}
/// QuerySmartContractStateResponse is the response type for the
/// Query/SmartContractState RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySmartContractStateResponse {
    /// Data contains the json data returned from the smart contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// grpc-gateway_out does not support Go style CodID
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
}
/// CodeInfoResponse contains code meta data from CodeInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfoResponse {
    /// id for legacy support
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// QueryCodeResponse is the response type for the Query/Code RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    #[prost(message, optional, tag = "1")]
    pub code_info: ::core::option::Option<CodeInfoResponse>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryCodesRequest is the request type for the Query/Codes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryCodesResponse is the response type for the Query/Codes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodesResponse {
    #[prost(message, repeated, tag = "1")]
    pub code_infos: ::prost::alloc::vec::Vec<CodeInfoResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPinnedCodesRequest is the request type for the Query/PinnedCodes
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPinnedCodesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPinnedCodesResponse is the response type for the
/// Query/PinnedCodes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPinnedCodesResponse {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
/// QueryContractsByCreatorRequest is the request type for the
/// Query/ContractsByCreator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCreatorRequest {
    /// CreatorAddress is the address of contract creator
    #[prost(string, tag = "1")]
    pub creator_address: ::prost::alloc::string::String,
    /// Pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryContractsByCreatorResponse is the response type for the
/// Query/ContractsByCreator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractsByCreatorResponse {
    /// ContractAddresses result set
    #[prost(string, repeated, tag = "1")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pagination defines the pagination in the response.
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
    /// Query provides defines the gRPC querier service
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
        /// ContractInfo gets the contract meta data
        pub async fn contract_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractInfoRequest>,
        ) -> Result<tonic::Response<super::QueryContractInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/ContractInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ContractHistory gets the contract code history
        pub async fn contract_history(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractHistoryRequest>,
        ) -> Result<tonic::Response<super::QueryContractHistoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/ContractHistory");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ContractsByCode lists all smart contracts for a code id
        pub async fn contracts_by_code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractsByCodeRequest>,
        ) -> Result<tonic::Response<super::QueryContractsByCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/ContractsByCode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AllContractState gets all raw store data for a single contract
        pub async fn all_contract_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllContractStateRequest>,
        ) -> Result<tonic::Response<super::QueryAllContractStateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/AllContractState");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RawContractState gets single key from the raw store data of a contract
        pub async fn raw_contract_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRawContractStateRequest>,
        ) -> Result<tonic::Response<super::QueryRawContractStateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/RawContractState");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SmartContractState get smart query result from the contract
        pub async fn smart_contract_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySmartContractStateRequest>,
        ) -> Result<tonic::Response<super::QuerySmartContractStateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/SmartContractState");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Code gets the binary code and metadata for a singe wasm code
        pub async fn code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCodeRequest>,
        ) -> Result<tonic::Response<super::QueryCodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/Code");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Codes gets the metadata for all stored wasm codes
        pub async fn codes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCodesRequest>,
        ) -> Result<tonic::Response<super::QueryCodesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/Codes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PinnedCodes gets the pinned code ids
        pub async fn pinned_codes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPinnedCodesRequest>,
        ) -> Result<tonic::Response<super::QueryPinnedCodesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/PinnedCodes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Params gets the module params
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
            let path = http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ContractsByCreator gets the contracts by creator
        pub async fn contracts_by_creator(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractsByCreatorRequest>,
        ) -> Result<tonic::Response<super::QueryContractsByCreatorResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmwasm.wasm.v1.Query/ContractsByCreator");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// MsgIBCSend
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcSend {
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "4")]
    pub timeout_height: u64,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "5")]
    pub timeout_timestamp: u64,
    /// Data is the payload to transfer. We must not make assumption what format or
    /// content is in here.
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgIBCCloseChannel port and channel need to be owned by the contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcCloseChannel {
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
/// StoreCodeProposal gov proposal content type to submit WASM code to the system
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreCodeProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "4")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "7")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional
    #[prost(bool, tag = "8")]
    pub unpin_code: bool,
}
/// InstantiateContractProposal gov proposal content type to instantiate a
/// contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "5")]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "7")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "8")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MigrateContractProposal gov proposal content type to migrate a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    ///
    /// Note: skipping 3 as this was previously used for unneeded run_as
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "5")]
    pub code_id: u64,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "6")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// SudoContractProposal gov proposal content type to call sudo on a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SudoContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as sudo
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// ExecuteContractProposal gov proposal content type to call execute on a
/// contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as execute
    #[prost(bytes = "vec", tag = "5")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// UpdateAdminProposal gov proposal content type to set an admin for a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
}
/// ClearAdminProposal gov proposal content type to clear the admin of a
/// contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
/// PinCodesProposal gov proposal content type to pin a set of code ids in the
/// wasmvm cache.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the new WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// UnpinCodesProposal gov proposal content type to unpin a set of code ids in
/// the wasmvm cache.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
/// AccessConfigUpdate contains the code id and the access config to be
/// applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfigUpdate {
    /// CodeID is the reference to the stored WASM code to be updated
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// InstantiatePermission to apply to the set of code ids
    #[prost(message, optional, tag = "2")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
/// UpdateInstantiateConfigProposal gov proposal content type to update
/// instantiate config to a  set of code ids.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstantiateConfigProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// AccessConfigUpdate contains the list of code ids and the access config
    /// to be applied.
    #[prost(message, repeated, tag = "3")]
    pub access_config_updates: ::prost::alloc::vec::Vec<AccessConfigUpdate>,
}
