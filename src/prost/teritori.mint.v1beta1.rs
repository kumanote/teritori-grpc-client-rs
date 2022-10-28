/// Minter represents the minting state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// current block provisions
    #[prost(string, tag="1")]
    pub block_provisions: ::prost::alloc::string::String,
}
/// required values for team rewards
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeamVestingMonthInfo {
    #[prost(int64, tag="1")]
    pub months_since_genesis: i64,
    #[prost(int64, tag="2")]
    pub month_started_block: i64,
    #[prost(int64, tag="3")]
    pub one_month_period_in_blocks: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyVestingAddress {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub monthly_amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionProportions {
    /// grants_program defines the proportion of the minted minted_denom that is
    /// to be allocated as grants.
    #[prost(string, tag="1")]
    pub grants_program: ::prost::alloc::string::String,
    /// community_pool defines the proportion of the minted minted_denom that is
    /// to be allocated to the community pool.
    #[prost(string, tag="2")]
    pub community_pool: ::prost::alloc::string::String,
    /// usage_incentive defines the proportion of the minted minted_denom that is
    /// to be allocated as usage incentive.
    #[prost(string, tag="3")]
    pub usage_incentive: ::prost::alloc::string::String,
    /// staking defines the proportion of the minted minted_denom that is to be
    /// allocated as staking rewards.
    #[prost(string, tag="4")]
    pub staking: ::prost::alloc::string::String,
    /// developer_rewards defines the proportion of the minted minted_denom that is
    /// to be allocated to developer rewards address.
    #[prost(string, tag="5")]
    pub developer_rewards: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// block provisions from the first block
    #[prost(string, tag="2")]
    pub genesis_block_provisions: ::prost::alloc::string::String,
    /// number of blocks take to reduce rewards
    #[prost(int64, tag="3")]
    pub reduction_period_in_blocks: i64,
    /// reduction multiplier to execute on each period
    #[prost(string, tag="4")]
    pub reduction_factor: ::prost::alloc::string::String,
    /// distribution_proportions defines the proportion of the minted denom
    #[prost(message, optional, tag="5")]
    pub distribution_proportions: ::core::option::Option<DistributionProportions>,
    /// address to receive developer rewards
    #[prost(message, repeated, tag="6")]
    pub weighted_developer_rewards_receivers: ::prost::alloc::vec::Vec<MonthlyVestingAddress>,
    /// usage incentive address
    #[prost(string, tag="7")]
    pub usage_incentive_address: ::prost::alloc::string::String,
    /// grants program address
    #[prost(string, tag="8")]
    pub grants_program_address: ::prost::alloc::string::String,
    /// team reserve funds address
    #[prost(string, tag="9")]
    pub team_reserve_address: ::prost::alloc::string::String,
    /// start block to distribute minting rewards
    #[prost(int64, tag="10")]
    pub minting_rewards_distribution_start_block: i64,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBlockProvisionsRequest is the request type for the
/// Query/BlockProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockProvisionsRequest {
}
/// QueryBlockProvisionsResponse is the response type for the
/// Query/BlockProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockProvisionsResponse {
    /// block_provisions is the current minting per epoch provisions value.
    #[prost(bytes="vec", tag="1")]
    pub block_provisions: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query provides defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
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
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
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
        /// Params returns the total set of minting parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/teritori.mint.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BlockProvisions current minting epoch provisions value.
        pub async fn block_provisions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBlockProvisionsRequest>,
        ) -> Result<
            tonic::Response<super::QueryBlockProvisionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/teritori.mint.v1beta1.Query/BlockProvisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the mint module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is a space for holding current rewards information.
    #[prost(message, optional, tag="1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
    /// required values for team rewards
    #[prost(message, optional, tag="3")]
    pub month_info: ::core::option::Option<TeamVestingMonthInfo>,
    /// current reduction period start block
    #[prost(int64, tag="4")]
    pub reduction_started_block: i64,
}
