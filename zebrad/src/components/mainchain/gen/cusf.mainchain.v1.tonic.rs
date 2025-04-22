// @generated
/// Generated client implementations.
pub mod validator_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ValidatorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ValidatorServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ValidatorServiceClient<T>
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
        ) -> ValidatorServiceClient<InterceptedService<T, F>>
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
            ValidatorServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_block_header_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeaderInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockHeaderInfoResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetBlockHeaderInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetBlockHeaderInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockInfoResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetBlockInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.ValidatorService", "GetBlockInfo"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_bmm_h_star_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBmmHStarCommitmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBmmHStarCommitmentResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetBmmHStarCommitment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetBmmHStarCommitment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_chain_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChainInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetChainInfoResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetChainInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.ValidatorService", "GetChainInfo"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_chain_tip(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChainTipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetChainTipResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetChainTip",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.ValidatorService", "GetChainTip"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_coinbase_psbt(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCoinbasePsbtRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCoinbasePsbtResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetCoinbasePSBT",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetCoinbasePSBT",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ctip(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCtipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCtipResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetCtip",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.ValidatorService", "GetCtip"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_sidechain_proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSidechainProposalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSidechainProposalsResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetSidechainProposals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetSidechainProposals",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_sidechains(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSidechainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSidechainsResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetSidechains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetSidechains",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_two_way_peg_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTwoWayPegDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTwoWayPegDataResponse>,
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
                "/cusf.mainchain.v1.ValidatorService/GetTwoWayPegData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "GetTwoWayPegData",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeEventsResponse>>,
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
                "/cusf.mainchain.v1.ValidatorService/SubscribeEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "SubscribeEvents",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_header_sync_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHeaderSyncProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::SubscribeHeaderSyncProgressResponse>,
            >,
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
                "/cusf.mainchain.v1.ValidatorService/SubscribeHeaderSyncProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.ValidatorService",
                        "SubscribeHeaderSyncProgress",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod wallet_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WalletServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WalletServiceClient<T>
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
        ) -> WalletServiceClient<InterceptedService<T, F>>
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
            WalletServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn broadcast_withdrawal_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastWithdrawalBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BroadcastWithdrawalBundleResponse>,
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
                "/cusf.mainchain.v1.WalletService/BroadcastWithdrawalBundle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "BroadcastWithdrawalBundle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_bmm_critical_data_transaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateBmmCriticalDataTransactionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateBmmCriticalDataTransactionResponse>,
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
                "/cusf.mainchain.v1.WalletService/CreateBmmCriticalDataTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "CreateBmmCriticalDataTransaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_deposit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDepositTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDepositTransactionResponse>,
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
                "/cusf.mainchain.v1.WalletService/CreateDepositTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "CreateDepositTransaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_new_address(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNewAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateNewAddressResponse>,
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
                "/cusf.mainchain.v1.WalletService/CreateNewAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "CreateNewAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_sidechain_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSidechainProposalRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::CreateSidechainProposalResponse>,
            >,
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
                "/cusf.mainchain.v1.WalletService/CreateSidechainProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "CreateSidechainProposal",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn create_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWalletResponse>,
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
                "/cusf.mainchain.v1.WalletService/CreateWallet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.WalletService", "CreateWallet"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBalanceResponse>,
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
                "/cusf.mainchain.v1.WalletService/GetBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.WalletService", "GetBalance"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_sidechain_deposit_transactions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListSidechainDepositTransactionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListSidechainDepositTransactionsResponse>,
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
                "/cusf.mainchain.v1.WalletService/ListSidechainDepositTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "ListSidechainDepositTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTransactionsResponse>,
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
                "/cusf.mainchain.v1.WalletService/ListTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "ListTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_unspent_outputs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUnspentOutputsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUnspentOutputsResponse>,
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
                "/cusf.mainchain.v1.WalletService/ListUnspentOutputs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "cusf.mainchain.v1.WalletService",
                        "ListUnspentOutputs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInfoResponse>,
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
                "/cusf.mainchain.v1.WalletService/GetInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cusf.mainchain.v1.WalletService", "GetInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendTransactionResponse>,
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
                "/cusf.mainchain.v1.WalletService/SendTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.WalletService", "SendTransaction"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockWalletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlockWalletResponse>,
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
                "/cusf.mainchain.v1.WalletService/UnlockWallet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.WalletService", "UnlockWallet"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateBlocksResponse>>,
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
                "/cusf.mainchain.v1.WalletService/GenerateBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("cusf.mainchain.v1.WalletService", "GenerateBlocks"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
