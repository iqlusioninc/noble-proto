// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
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
            D: TryInto<tonic::transport::Endpoint>,
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
        pub async fn roles(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRolesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryRolesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Roles");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Roles"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn attester(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAttesterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Attester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Attester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn attesters(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllAttestersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllAttestersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Attesters");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Attesters"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn per_message_burn_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPerMessageBurnLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPerMessageBurnLimitResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/PerMessageBurnLimit");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "PerMessageBurnLimit",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn per_message_burn_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllPerMessageBurnLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllPerMessageBurnLimitsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/PerMessageBurnLimits");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "PerMessageBurnLimits",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burning_and_minting_paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBurningAndMintingPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetBurningAndMintingPausedResponse>,
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
                "/circle.cctp.v1.Query/BurningAndMintingPaused",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "BurningAndMintingPaused",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sending_and_receiving_messages_paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetSendingAndReceivingMessagesPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSendingAndReceivingMessagesPausedResponse>,
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
                "/circle.cctp.v1.Query/SendingAndReceivingMessagesPaused",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "SendingAndReceivingMessagesPaused",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn max_message_body_size(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMaxMessageBodySizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMaxMessageBodySizeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/MaxMessageBodySize");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "MaxMessageBodySize",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn next_available_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetNextAvailableNonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetNextAvailableNonceResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/NextAvailableNonce");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "NextAvailableNonce",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn signature_threshold(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetSignatureThresholdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSignatureThresholdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/SignatureThreshold");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "SignatureThreshold",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetTokenPairRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/TokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "TokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn token_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllTokenPairsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllTokenPairsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/TokenPairs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "TokenPairs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn used_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetUsedNonceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetUsedNonceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/UsedNonce");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "UsedNonce"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn used_nonces(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllUsedNoncesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllUsedNoncesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/UsedNonces");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "UsedNonces"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRemoteTokenMessengerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/RemoteTokenMessenger");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "RemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remote_token_messengers(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRemoteTokenMessengersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/RemoteTokenMessengers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "RemoteTokenMessengers",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burn_message_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBurnMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBurnMessageVersionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/BurnMessageVersion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "BurnMessageVersion",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn local_message_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLocalMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryLocalMessageVersionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/LocalMessageVersion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "LocalMessageVersion",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn local_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLocalDomainRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryLocalDomainResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/LocalDomain");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "LocalDomain"));
            self.inner.unary(req, path, codec).await
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
        async fn roles(
            &self,
            request: tonic::Request<super::QueryRolesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryRolesResponse>, tonic::Status>;
        async fn attester(
            &self,
            request: tonic::Request<super::QueryGetAttesterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAttesterResponse>, tonic::Status>;
        async fn attesters(
            &self,
            request: tonic::Request<super::QueryAllAttestersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllAttestersResponse>, tonic::Status>;
        async fn per_message_burn_limit(
            &self,
            request: tonic::Request<super::QueryGetPerMessageBurnLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPerMessageBurnLimitResponse>,
            tonic::Status,
        >;
        async fn per_message_burn_limits(
            &self,
            request: tonic::Request<super::QueryAllPerMessageBurnLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllPerMessageBurnLimitsResponse>,
            tonic::Status,
        >;
        async fn burning_and_minting_paused(
            &self,
            request: tonic::Request<super::QueryGetBurningAndMintingPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetBurningAndMintingPausedResponse>,
            tonic::Status,
        >;
        async fn sending_and_receiving_messages_paused(
            &self,
            request: tonic::Request<super::QueryGetSendingAndReceivingMessagesPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSendingAndReceivingMessagesPausedResponse>,
            tonic::Status,
        >;
        async fn max_message_body_size(
            &self,
            request: tonic::Request<super::QueryGetMaxMessageBodySizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMaxMessageBodySizeResponse>,
            tonic::Status,
        >;
        async fn next_available_nonce(
            &self,
            request: tonic::Request<super::QueryGetNextAvailableNonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetNextAvailableNonceResponse>,
            tonic::Status,
        >;
        async fn signature_threshold(
            &self,
            request: tonic::Request<super::QueryGetSignatureThresholdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSignatureThresholdResponse>,
            tonic::Status,
        >;
        async fn token_pair(
            &self,
            request: tonic::Request<super::QueryGetTokenPairRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetTokenPairResponse>, tonic::Status>;
        async fn token_pairs(
            &self,
            request: tonic::Request<super::QueryAllTokenPairsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllTokenPairsResponse>, tonic::Status>;
        async fn used_nonce(
            &self,
            request: tonic::Request<super::QueryGetUsedNonceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetUsedNonceResponse>, tonic::Status>;
        async fn used_nonces(
            &self,
            request: tonic::Request<super::QueryAllUsedNoncesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllUsedNoncesResponse>, tonic::Status>;
        async fn remote_token_messenger(
            &self,
            request: tonic::Request<super::QueryRemoteTokenMessengerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengerResponse>,
            tonic::Status,
        >;
        async fn remote_token_messengers(
            &self,
            request: tonic::Request<super::QueryRemoteTokenMessengersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengersResponse>,
            tonic::Status,
        >;
        async fn burn_message_version(
            &self,
            request: tonic::Request<super::QueryBurnMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBurnMessageVersionResponse>,
            tonic::Status,
        >;
        async fn local_message_version(
            &self,
            request: tonic::Request<super::QueryLocalMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryLocalMessageVersionResponse>,
            tonic::Status,
        >;
        async fn local_domain(
            &self,
            request: tonic::Request<super::QueryLocalDomainRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryLocalDomainResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
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
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/circle.cctp.v1.Query/Roles" => {
                    #[allow(non_camel_case_types)]
                    struct RolesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRolesRequest> for RolesSvc<T> {
                        type Response = super::QueryRolesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRolesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::roles(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/Attester" => {
                    #[allow(non_camel_case_types)]
                    struct AttesterSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetAttesterRequest> for AttesterSvc<T> {
                        type Response = super::QueryGetAttesterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetAttesterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::attester(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AttesterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/Attesters" => {
                    #[allow(non_camel_case_types)]
                    struct AttestersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllAttestersRequest> for AttestersSvc<T> {
                        type Response = super::QueryAllAttestersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllAttestersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::attesters(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AttestersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/PerMessageBurnLimit" => {
                    #[allow(non_camel_case_types)]
                    struct PerMessageBurnLimitSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetPerMessageBurnLimitRequest>
                        for PerMessageBurnLimitSvc<T>
                    {
                        type Response = super::QueryGetPerMessageBurnLimitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPerMessageBurnLimitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::per_message_burn_limit(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PerMessageBurnLimitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/PerMessageBurnLimits" => {
                    #[allow(non_camel_case_types)]
                    struct PerMessageBurnLimitsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllPerMessageBurnLimitsRequest>
                        for PerMessageBurnLimitsSvc<T>
                    {
                        type Response = super::QueryAllPerMessageBurnLimitsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllPerMessageBurnLimitsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::per_message_burn_limits(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PerMessageBurnLimitsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/BurningAndMintingPaused" => {
                    #[allow(non_camel_case_types)]
                    struct BurningAndMintingPausedSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetBurningAndMintingPausedRequest>
                        for BurningAndMintingPausedSvc<T>
                    {
                        type Response = super::QueryGetBurningAndMintingPausedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetBurningAndMintingPausedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::burning_and_minting_paused(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BurningAndMintingPausedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/SendingAndReceivingMessagesPaused" => {
                    #[allow(non_camel_case_types)]
                    struct SendingAndReceivingMessagesPausedSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryGetSendingAndReceivingMessagesPausedRequest,
                        > for SendingAndReceivingMessagesPausedSvc<T>
                    {
                        type Response = super::QueryGetSendingAndReceivingMessagesPausedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryGetSendingAndReceivingMessagesPausedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::sending_and_receiving_messages_paused(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendingAndReceivingMessagesPausedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/MaxMessageBodySize" => {
                    #[allow(non_camel_case_types)]
                    struct MaxMessageBodySizeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetMaxMessageBodySizeRequest>
                        for MaxMessageBodySizeSvc<T>
                    {
                        type Response = super::QueryGetMaxMessageBodySizeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMaxMessageBodySizeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::max_message_body_size(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MaxMessageBodySizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/NextAvailableNonce" => {
                    #[allow(non_camel_case_types)]
                    struct NextAvailableNonceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetNextAvailableNonceRequest>
                        for NextAvailableNonceSvc<T>
                    {
                        type Response = super::QueryGetNextAvailableNonceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetNextAvailableNonceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::next_available_nonce(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = NextAvailableNonceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/SignatureThreshold" => {
                    #[allow(non_camel_case_types)]
                    struct SignatureThresholdSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetSignatureThresholdRequest>
                        for SignatureThresholdSvc<T>
                    {
                        type Response = super::QueryGetSignatureThresholdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetSignatureThresholdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::signature_threshold(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SignatureThresholdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/TokenPair" => {
                    #[allow(non_camel_case_types)]
                    struct TokenPairSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetTokenPairRequest> for TokenPairSvc<T> {
                        type Response = super::QueryGetTokenPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetTokenPairRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::token_pair(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TokenPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/TokenPairs" => {
                    #[allow(non_camel_case_types)]
                    struct TokenPairsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllTokenPairsRequest> for TokenPairsSvc<T> {
                        type Response = super::QueryAllTokenPairsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllTokenPairsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::token_pairs(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TokenPairsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/UsedNonce" => {
                    #[allow(non_camel_case_types)]
                    struct UsedNonceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetUsedNonceRequest> for UsedNonceSvc<T> {
                        type Response = super::QueryGetUsedNonceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetUsedNonceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::used_nonce(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UsedNonceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/UsedNonces" => {
                    #[allow(non_camel_case_types)]
                    struct UsedNoncesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllUsedNoncesRequest> for UsedNoncesSvc<T> {
                        type Response = super::QueryAllUsedNoncesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllUsedNoncesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::used_nonces(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UsedNoncesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/RemoteTokenMessenger" => {
                    #[allow(non_camel_case_types)]
                    struct RemoteTokenMessengerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRemoteTokenMessengerRequest>
                        for RemoteTokenMessengerSvc<T>
                    {
                        type Response = super::QueryRemoteTokenMessengerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRemoteTokenMessengerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::remote_token_messenger(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoteTokenMessengerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/RemoteTokenMessengers" => {
                    #[allow(non_camel_case_types)]
                    struct RemoteTokenMessengersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRemoteTokenMessengersRequest>
                        for RemoteTokenMessengersSvc<T>
                    {
                        type Response = super::QueryRemoteTokenMessengersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRemoteTokenMessengersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::remote_token_messengers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoteTokenMessengersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/BurnMessageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct BurnMessageVersionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryBurnMessageVersionRequest>
                        for BurnMessageVersionSvc<T>
                    {
                        type Response = super::QueryBurnMessageVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBurnMessageVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::burn_message_version(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BurnMessageVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/LocalMessageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct LocalMessageVersionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryLocalMessageVersionRequest>
                        for LocalMessageVersionSvc<T>
                    {
                        type Response = super::QueryLocalMessageVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLocalMessageVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::local_message_version(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LocalMessageVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Query/LocalDomain" => {
                    #[allow(non_camel_case_types)]
                    struct LocalDomainSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLocalDomainRequest> for LocalDomainSvc<T> {
                        type Response = super::QueryLocalDomainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLocalDomainRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::local_domain(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LocalDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", tonic::Code::Unimplemented as i32)
                        .header(
                            http::header::CONTENT_TYPE,
                            tonic::metadata::GRPC_CONTENT_TYPE,
                        )
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
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "circle.cctp.v1.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
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
            D: TryInto<tonic::transport::Endpoint>,
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
        pub async fn accept_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAcceptOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/AcceptOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "AcceptOwner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddRemoteTokenMessengerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/AddRemoteTokenMessenger");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "AddRemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deposit_for_burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositForBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgDepositForBurnResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/DepositForBurn");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "DepositForBurn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deposit_for_burn_with_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositForBurnWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositForBurnWithCallerResponse>,
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
                "/circle.cctp.v1.Msg/DepositForBurnWithCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "DepositForBurnWithCaller",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disable_attester(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDisableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgDisableAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/DisableAttester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "DisableAttester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn enable_attester(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEnableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgEnableAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/EnableAttester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "EnableAttester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn link_token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgLinkTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/LinkTokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "LinkTokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause_burning_and_minting(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseBurningAndMintingResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/PauseBurningAndMinting");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "PauseBurningAndMinting",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause_sending_and_receiving_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseSendingAndReceivingMessagesResponse>,
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
                "/circle.cctp.v1.Msg/PauseSendingAndReceivingMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "PauseSendingAndReceivingMessages",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReceiveMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReceiveMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReceiveMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "ReceiveMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveRemoteTokenMessengerResponse>,
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
                "/circle.cctp.v1.Msg/RemoveRemoteTokenMessenger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "RemoveRemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn replace_deposit_for_burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReplaceDepositForBurn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgReplaceDepositForBurnResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReplaceDepositForBurn");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "ReplaceDepositForBurn",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn replace_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReplaceMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReplaceMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReplaceMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "ReplaceMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgSendMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/SendMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "SendMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_message_with_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendMessageWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSendMessageWithCallerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/SendMessageWithCaller");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "SendMessageWithCaller",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlink_token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnlinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgUnlinkTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UnlinkTokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UnlinkTokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause_burning_and_minting(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseBurningAndMintingResponse>,
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
                "/circle.cctp.v1.Msg/UnpauseBurningAndMinting",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UnpauseBurningAndMinting",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause_sending_and_receiving_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseSendingAndReceivingMessagesResponse>,
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
                "/circle.cctp.v1.Msg/UnpauseSendingAndReceivingMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UnpauseSendingAndReceivingMessages",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UpdateOwner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_attester_manager(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAttesterManager>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAttesterManagerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateAttesterManager");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateAttesterManager",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_token_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateTokenController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateTokenControllerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateTokenController");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateTokenController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_pauser(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdatePauser>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdatePauserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdatePauser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UpdatePauser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_max_message_body_size(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateMaxMessageBodySize>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateMaxMessageBodySizeResponse>,
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
                "/circle.cctp.v1.Msg/UpdateMaxMessageBodySize",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateMaxMessageBodySize",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_max_burn_amount_per_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetMaxBurnAmountPerMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxBurnAmountPerMessageResponse>,
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
                "/circle.cctp.v1.Msg/SetMaxBurnAmountPerMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "SetMaxBurnAmountPerMessage",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_signature_threshold(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateSignatureThreshold>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateSignatureThresholdResponse>,
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
                "/circle.cctp.v1.Msg/UpdateSignatureThreshold",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateSignatureThreshold",
            ));
            self.inner.unary(req, path, codec).await
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
        async fn accept_owner(
            &self,
            request: tonic::Request<super::MsgAcceptOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptOwnerResponse>, tonic::Status>;
        async fn add_remote_token_messenger(
            &self,
            request: tonic::Request<super::MsgAddRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddRemoteTokenMessengerResponse>,
            tonic::Status,
        >;
        async fn deposit_for_burn(
            &self,
            request: tonic::Request<super::MsgDepositForBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgDepositForBurnResponse>, tonic::Status>;
        async fn deposit_for_burn_with_caller(
            &self,
            request: tonic::Request<super::MsgDepositForBurnWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositForBurnWithCallerResponse>,
            tonic::Status,
        >;
        async fn disable_attester(
            &self,
            request: tonic::Request<super::MsgDisableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgDisableAttesterResponse>, tonic::Status>;
        async fn enable_attester(
            &self,
            request: tonic::Request<super::MsgEnableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgEnableAttesterResponse>, tonic::Status>;
        async fn link_token_pair(
            &self,
            request: tonic::Request<super::MsgLinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgLinkTokenPairResponse>, tonic::Status>;
        async fn pause_burning_and_minting(
            &self,
            request: tonic::Request<super::MsgPauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseBurningAndMintingResponse>,
            tonic::Status,
        >;
        async fn pause_sending_and_receiving_messages(
            &self,
            request: tonic::Request<super::MsgPauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseSendingAndReceivingMessagesResponse>,
            tonic::Status,
        >;
        async fn receive_message(
            &self,
            request: tonic::Request<super::MsgReceiveMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReceiveMessageResponse>, tonic::Status>;
        async fn remove_remote_token_messenger(
            &self,
            request: tonic::Request<super::MsgRemoveRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveRemoteTokenMessengerResponse>,
            tonic::Status,
        >;
        async fn replace_deposit_for_burn(
            &self,
            request: tonic::Request<super::MsgReplaceDepositForBurn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgReplaceDepositForBurnResponse>,
            tonic::Status,
        >;
        async fn replace_message(
            &self,
            request: tonic::Request<super::MsgReplaceMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReplaceMessageResponse>, tonic::Status>;
        async fn send_message(
            &self,
            request: tonic::Request<super::MsgSendMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgSendMessageResponse>, tonic::Status>;
        async fn send_message_with_caller(
            &self,
            request: tonic::Request<super::MsgSendMessageWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSendMessageWithCallerResponse>,
            tonic::Status,
        >;
        async fn unlink_token_pair(
            &self,
            request: tonic::Request<super::MsgUnlinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgUnlinkTokenPairResponse>, tonic::Status>;
        async fn unpause_burning_and_minting(
            &self,
            request: tonic::Request<super::MsgUnpauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseBurningAndMintingResponse>,
            tonic::Status,
        >;
        async fn unpause_sending_and_receiving_messages(
            &self,
            request: tonic::Request<super::MsgUnpauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseSendingAndReceivingMessagesResponse>,
            tonic::Status,
        >;
        async fn update_owner(
            &self,
            request: tonic::Request<super::MsgUpdateOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateOwnerResponse>, tonic::Status>;
        async fn update_attester_manager(
            &self,
            request: tonic::Request<super::MsgUpdateAttesterManager>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAttesterManagerResponse>,
            tonic::Status,
        >;
        async fn update_token_controller(
            &self,
            request: tonic::Request<super::MsgUpdateTokenController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateTokenControllerResponse>,
            tonic::Status,
        >;
        async fn update_pauser(
            &self,
            request: tonic::Request<super::MsgUpdatePauser>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdatePauserResponse>, tonic::Status>;
        async fn update_max_message_body_size(
            &self,
            request: tonic::Request<super::MsgUpdateMaxMessageBodySize>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateMaxMessageBodySizeResponse>,
            tonic::Status,
        >;
        async fn set_max_burn_amount_per_message(
            &self,
            request: tonic::Request<super::MsgSetMaxBurnAmountPerMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxBurnAmountPerMessageResponse>,
            tonic::Status,
        >;
        async fn update_signature_threshold(
            &self,
            request: tonic::Request<super::MsgUpdateSignatureThreshold>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateSignatureThresholdResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
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
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/circle.cctp.v1.Msg/AcceptOwner" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAcceptOwner> for AcceptOwnerSvc<T> {
                        type Response = super::MsgAcceptOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAcceptOwner>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::accept_owner(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AcceptOwnerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/AddRemoteTokenMessenger" => {
                    #[allow(non_camel_case_types)]
                    struct AddRemoteTokenMessengerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddRemoteTokenMessenger>
                        for AddRemoteTokenMessengerSvc<T>
                    {
                        type Response = super::MsgAddRemoteTokenMessengerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddRemoteTokenMessenger>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::add_remote_token_messenger(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AddRemoteTokenMessengerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/DepositForBurn" => {
                    #[allow(non_camel_case_types)]
                    struct DepositForBurnSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositForBurn> for DepositForBurnSvc<T> {
                        type Response = super::MsgDepositForBurnResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositForBurn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::deposit_for_burn(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DepositForBurnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/DepositForBurnWithCaller" => {
                    #[allow(non_camel_case_types)]
                    struct DepositForBurnWithCallerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDepositForBurnWithCaller>
                        for DepositForBurnWithCallerSvc<T>
                    {
                        type Response = super::MsgDepositForBurnWithCallerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDepositForBurnWithCaller>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::deposit_for_burn_with_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DepositForBurnWithCallerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/DisableAttester" => {
                    #[allow(non_camel_case_types)]
                    struct DisableAttesterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDisableAttester> for DisableAttesterSvc<T> {
                        type Response = super::MsgDisableAttesterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDisableAttester>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::disable_attester(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DisableAttesterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/EnableAttester" => {
                    #[allow(non_camel_case_types)]
                    struct EnableAttesterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgEnableAttester> for EnableAttesterSvc<T> {
                        type Response = super::MsgEnableAttesterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgEnableAttester>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::enable_attester(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = EnableAttesterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/LinkTokenPair" => {
                    #[allow(non_camel_case_types)]
                    struct LinkTokenPairSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgLinkTokenPair> for LinkTokenPairSvc<T> {
                        type Response = super::MsgLinkTokenPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgLinkTokenPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::link_token_pair(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LinkTokenPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/PauseBurningAndMinting" => {
                    #[allow(non_camel_case_types)]
                    struct PauseBurningAndMintingSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPauseBurningAndMinting>
                        for PauseBurningAndMintingSvc<T>
                    {
                        type Response = super::MsgPauseBurningAndMintingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPauseBurningAndMinting>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::pause_burning_and_minting(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PauseBurningAndMintingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/PauseSendingAndReceivingMessages" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSendingAndReceivingMessagesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgPauseSendingAndReceivingMessages>
                        for PauseSendingAndReceivingMessagesSvc<T>
                    {
                        type Response = super::MsgPauseSendingAndReceivingMessagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPauseSendingAndReceivingMessages>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::pause_sending_and_receiving_messages(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PauseSendingAndReceivingMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/ReceiveMessage" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveMessageSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgReceiveMessage> for ReceiveMessageSvc<T> {
                        type Response = super::MsgReceiveMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgReceiveMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::receive_message(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReceiveMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/RemoveRemoteTokenMessenger" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveRemoteTokenMessengerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRemoveRemoteTokenMessenger>
                        for RemoveRemoteTokenMessengerSvc<T>
                    {
                        type Response = super::MsgRemoveRemoteTokenMessengerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRemoveRemoteTokenMessenger>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::remove_remote_token_messenger(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveRemoteTokenMessengerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/ReplaceDepositForBurn" => {
                    #[allow(non_camel_case_types)]
                    struct ReplaceDepositForBurnSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgReplaceDepositForBurn>
                        for ReplaceDepositForBurnSvc<T>
                    {
                        type Response = super::MsgReplaceDepositForBurnResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgReplaceDepositForBurn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::replace_deposit_for_burn(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReplaceDepositForBurnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/ReplaceMessage" => {
                    #[allow(non_camel_case_types)]
                    struct ReplaceMessageSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgReplaceMessage> for ReplaceMessageSvc<T> {
                        type Response = super::MsgReplaceMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgReplaceMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::replace_message(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReplaceMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSendMessage> for SendMessageSvc<T> {
                        type Response = super::MsgSendMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSendMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::send_message(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/SendMessageWithCaller" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageWithCallerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSendMessageWithCaller>
                        for SendMessageWithCallerSvc<T>
                    {
                        type Response = super::MsgSendMessageWithCallerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSendMessageWithCaller>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::send_message_with_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendMessageWithCallerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UnlinkTokenPair" => {
                    #[allow(non_camel_case_types)]
                    struct UnlinkTokenPairSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnlinkTokenPair> for UnlinkTokenPairSvc<T> {
                        type Response = super::MsgUnlinkTokenPairResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnlinkTokenPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::unlink_token_pair(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnlinkTokenPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UnpauseBurningAndMinting" => {
                    #[allow(non_camel_case_types)]
                    struct UnpauseBurningAndMintingSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnpauseBurningAndMinting>
                        for UnpauseBurningAndMintingSvc<T>
                    {
                        type Response = super::MsgUnpauseBurningAndMintingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnpauseBurningAndMinting>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::unpause_burning_and_minting(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnpauseBurningAndMintingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UnpauseSendingAndReceivingMessages" => {
                    #[allow(non_camel_case_types)]
                    struct UnpauseSendingAndReceivingMessagesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgUnpauseSendingAndReceivingMessages>
                        for UnpauseSendingAndReceivingMessagesSvc<T>
                    {
                        type Response = super::MsgUnpauseSendingAndReceivingMessagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnpauseSendingAndReceivingMessages>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::unpause_sending_and_receiving_messages(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnpauseSendingAndReceivingMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdateOwner" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateOwner> for UpdateOwnerSvc<T> {
                        type Response = super::MsgUpdateOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateOwner>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::update_owner(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateOwnerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdateAttesterManager" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAttesterManagerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateAttesterManager>
                        for UpdateAttesterManagerSvc<T>
                    {
                        type Response = super::MsgUpdateAttesterManagerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateAttesterManager>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_attester_manager(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateAttesterManagerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdateTokenController" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTokenControllerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateTokenController>
                        for UpdateTokenControllerSvc<T>
                    {
                        type Response = super::MsgUpdateTokenControllerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateTokenController>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_token_controller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateTokenControllerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdatePauser" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePauserSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdatePauser> for UpdatePauserSvc<T> {
                        type Response = super::MsgUpdatePauserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdatePauser>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::update_pauser(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdatePauserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdateMaxMessageBodySize" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMaxMessageBodySizeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateMaxMessageBodySize>
                        for UpdateMaxMessageBodySizeSvc<T>
                    {
                        type Response = super::MsgUpdateMaxMessageBodySizeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateMaxMessageBodySize>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_max_message_body_size(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateMaxMessageBodySizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/SetMaxBurnAmountPerMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SetMaxBurnAmountPerMessageSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetMaxBurnAmountPerMessage>
                        for SetMaxBurnAmountPerMessageSvc<T>
                    {
                        type Response = super::MsgSetMaxBurnAmountPerMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetMaxBurnAmountPerMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::set_max_burn_amount_per_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SetMaxBurnAmountPerMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/circle.cctp.v1.Msg/UpdateSignatureThreshold" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSignatureThresholdSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateSignatureThreshold>
                        for UpdateSignatureThresholdSvc<T>
                    {
                        type Response = super::MsgUpdateSignatureThresholdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateSignatureThreshold>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_signature_threshold(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateSignatureThresholdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", tonic::Code::Unimplemented as i32)
                        .header(
                            http::header::CONTENT_TYPE,
                            tonic::metadata::GRPC_CONTENT_TYPE,
                        )
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
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "circle.cctp.v1.Msg";
    }
}
