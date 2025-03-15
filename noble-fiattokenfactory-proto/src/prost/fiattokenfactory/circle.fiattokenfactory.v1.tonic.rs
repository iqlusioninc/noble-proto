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
        pub async fn blacklisted(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklistedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/Blacklisted",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "Blacklisted",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklisted_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllBlacklistedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/BlacklistedAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "BlacklistedAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPausedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPausedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Query/Paused");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "Paused",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn master_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMasterMinterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMasterMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/MasterMinter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "MasterMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minters(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Query/Minters");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "Minters",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minters_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllMintersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/MintersAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "MintersAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pauser(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPauserRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPauserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Query/Pauser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "Pauser",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklister(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBlacklisterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/Blacklister",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "Blacklister",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Query/Owner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.fiattokenfactory.v1.Query", "Owner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMinterControllerResponse>,
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
                "/circle.fiattokenfactory.v1.Query/MinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "MinterController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minter_controller_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllMinterControllerResponse>,
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
                "/circle.fiattokenfactory.v1.Query/MinterControllerAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "MinterControllerAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minting_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMintingDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintingDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Query/MintingDenom",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Query",
                "MintingDenom",
            ));
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
        async fn blacklisted(
            &self,
            request: tonic::Request<super::QueryGetBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklistedResponse>, tonic::Status>;
        async fn blacklisted_all(
            &self,
            request: tonic::Request<super::QueryAllBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllBlacklistedResponse>, tonic::Status>;
        async fn paused(
            &self,
            request: tonic::Request<super::QueryGetPausedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPausedResponse>, tonic::Status>;
        async fn master_minter(
            &self,
            request: tonic::Request<super::QueryGetMasterMinterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMasterMinterResponse>, tonic::Status>;
        async fn minters(
            &self,
            request: tonic::Request<super::QueryGetMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintersResponse>, tonic::Status>;
        async fn minters_all(
            &self,
            request: tonic::Request<super::QueryAllMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllMintersResponse>, tonic::Status>;
        async fn pauser(
            &self,
            request: tonic::Request<super::QueryGetPauserRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPauserResponse>, tonic::Status>;
        async fn blacklister(
            &self,
            request: tonic::Request<super::QueryGetBlacklisterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklisterResponse>, tonic::Status>;
        async fn owner(
            &self,
            request: tonic::Request<super::QueryGetOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOwnerResponse>, tonic::Status>;
        async fn minter_controller(
            &self,
            request: tonic::Request<super::QueryGetMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMinterControllerResponse>,
            tonic::Status,
        >;
        async fn minter_controller_all(
            &self,
            request: tonic::Request<super::QueryAllMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllMinterControllerResponse>,
            tonic::Status,
        >;
        async fn minting_denom(
            &self,
            request: tonic::Request<super::QueryGetMintingDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintingDenomResponse>, tonic::Status>;
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
                "/circle.fiattokenfactory.v1.Query/Blacklisted" => {
                    #[allow(non_camel_case_types)]
                    struct BlacklistedSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetBlacklistedRequest>
                        for BlacklistedSvc<T>
                    {
                        type Response = super::QueryGetBlacklistedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetBlacklistedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::blacklisted(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BlacklistedSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/BlacklistedAll" => {
                    #[allow(non_camel_case_types)]
                    struct BlacklistedAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllBlacklistedRequest>
                        for BlacklistedAllSvc<T>
                    {
                        type Response = super::QueryAllBlacklistedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllBlacklistedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::blacklisted_all(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BlacklistedAllSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/Paused" => {
                    #[allow(non_camel_case_types)]
                    struct PausedSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetPausedRequest> for PausedSvc<T> {
                        type Response = super::QueryGetPausedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPausedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::paused(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PausedSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/MasterMinter" => {
                    #[allow(non_camel_case_types)]
                    struct MasterMinterSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetMasterMinterRequest>
                        for MasterMinterSvc<T>
                    {
                        type Response = super::QueryGetMasterMinterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMasterMinterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::master_minter(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MasterMinterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/Minters" => {
                    #[allow(non_camel_case_types)]
                    struct MintersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetMintersRequest> for MintersSvc<T> {
                        type Response = super::QueryGetMintersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMintersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::minters(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MintersSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/MintersAll" => {
                    #[allow(non_camel_case_types)]
                    struct MintersAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllMintersRequest> for MintersAllSvc<T> {
                        type Response = super::QueryAllMintersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllMintersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::minters_all(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MintersAllSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/Pauser" => {
                    #[allow(non_camel_case_types)]
                    struct PauserSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetPauserRequest> for PauserSvc<T> {
                        type Response = super::QueryGetPauserResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetPauserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::pauser(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PauserSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/Blacklister" => {
                    #[allow(non_camel_case_types)]
                    struct BlacklisterSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetBlacklisterRequest>
                        for BlacklisterSvc<T>
                    {
                        type Response = super::QueryGetBlacklisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetBlacklisterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::blacklister(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BlacklisterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/Owner" => {
                    #[allow(non_camel_case_types)]
                    struct OwnerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetOwnerRequest> for OwnerSvc<T> {
                        type Response = super::QueryGetOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetOwnerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Query>::owner(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = OwnerSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/MinterController" => {
                    #[allow(non_camel_case_types)]
                    struct MinterControllerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryGetMinterControllerRequest>
                        for MinterControllerSvc<T>
                    {
                        type Response = super::QueryGetMinterControllerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMinterControllerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::minter_controller(&inner, request).await
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
                        let method = MinterControllerSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/MinterControllerAll" => {
                    #[allow(non_camel_case_types)]
                    struct MinterControllerAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryAllMinterControllerRequest>
                        for MinterControllerAllSvc<T>
                    {
                        type Response = super::QueryAllMinterControllerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllMinterControllerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::minter_controller_all(&inner, request).await
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
                        let method = MinterControllerAllSvc(inner);
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
                "/circle.fiattokenfactory.v1.Query/MintingDenom" => {
                    #[allow(non_camel_case_types)]
                    struct MintingDenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryGetMintingDenomRequest>
                        for MintingDenomSvc<T>
                    {
                        type Response = super::QueryGetMintingDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGetMintingDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Query>::minting_denom(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MintingDenomSvc(inner);
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
        const NAME: &'static str = "circle.fiattokenfactory.v1.Query";
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
        pub async fn update_master_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateMasterMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateMasterMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Msg/UpdateMasterMinter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "UpdateMasterMinter",
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
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Msg/UpdatePauser",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "UpdatePauser",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_blacklister(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateBlacklister>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateBlacklisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Msg/UpdateBlacklister",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "UpdateBlacklister",
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
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/UpdateOwner");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "UpdateOwner",
            ));
            self.inner.unary(req, path, codec).await
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
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/AcceptOwner");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "AcceptOwner",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfigureMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgConfigureMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Msg/ConfigureMinter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "ConfigureMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgRemoveMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.fiattokenfactory.v1.Msg/RemoveMinter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "RemoveMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMint>,
        ) -> std::result::Result<tonic::Response<super::MsgMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Mint");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.fiattokenfactory.v1.Msg", "Mint"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgBurnResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Burn");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.fiattokenfactory.v1.Msg", "Burn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBlacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgBlacklistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Blacklist");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "Blacklist",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unblacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnblacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgUnblacklistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Unblacklist");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "Unblacklist",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPause>,
        ) -> std::result::Result<tonic::Response<super::MsgPauseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Pause");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.fiattokenfactory.v1.Msg", "Pause"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpause>,
        ) -> std::result::Result<tonic::Response<super::MsgUnpauseResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.fiattokenfactory.v1.Msg/Unpause");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.fiattokenfactory.v1.Msg", "Unpause"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfigureMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgConfigureMinterControllerResponse>,
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
                "/circle.fiattokenfactory.v1.Msg/ConfigureMinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "ConfigureMinterController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveMinterControllerResponse>,
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
                "/circle.fiattokenfactory.v1.Msg/RemoveMinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.fiattokenfactory.v1.Msg",
                "RemoveMinterController",
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
        async fn update_master_minter(
            &self,
            request: tonic::Request<super::MsgUpdateMasterMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateMasterMinterResponse>, tonic::Status>;
        async fn update_pauser(
            &self,
            request: tonic::Request<super::MsgUpdatePauser>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdatePauserResponse>, tonic::Status>;
        async fn update_blacklister(
            &self,
            request: tonic::Request<super::MsgUpdateBlacklister>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateBlacklisterResponse>, tonic::Status>;
        async fn update_owner(
            &self,
            request: tonic::Request<super::MsgUpdateOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateOwnerResponse>, tonic::Status>;
        async fn accept_owner(
            &self,
            request: tonic::Request<super::MsgAcceptOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptOwnerResponse>, tonic::Status>;
        async fn configure_minter(
            &self,
            request: tonic::Request<super::MsgConfigureMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgConfigureMinterResponse>, tonic::Status>;
        async fn remove_minter(
            &self,
            request: tonic::Request<super::MsgRemoveMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgRemoveMinterResponse>, tonic::Status>;
        async fn mint(
            &self,
            request: tonic::Request<super::MsgMint>,
        ) -> std::result::Result<tonic::Response<super::MsgMintResponse>, tonic::Status>;
        async fn burn(
            &self,
            request: tonic::Request<super::MsgBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgBurnResponse>, tonic::Status>;
        async fn blacklist(
            &self,
            request: tonic::Request<super::MsgBlacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgBlacklistResponse>, tonic::Status>;
        async fn unblacklist(
            &self,
            request: tonic::Request<super::MsgUnblacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgUnblacklistResponse>, tonic::Status>;
        async fn pause(
            &self,
            request: tonic::Request<super::MsgPause>,
        ) -> std::result::Result<tonic::Response<super::MsgPauseResponse>, tonic::Status>;
        async fn unpause(
            &self,
            request: tonic::Request<super::MsgUnpause>,
        ) -> std::result::Result<tonic::Response<super::MsgUnpauseResponse>, tonic::Status>;
        async fn configure_minter_controller(
            &self,
            request: tonic::Request<super::MsgConfigureMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgConfigureMinterControllerResponse>,
            tonic::Status,
        >;
        async fn remove_minter_controller(
            &self,
            request: tonic::Request<super::MsgRemoveMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveMinterControllerResponse>,
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
                "/circle.fiattokenfactory.v1.Msg/UpdateMasterMinter" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMasterMinterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateMasterMinter>
                        for UpdateMasterMinterSvc<T>
                    {
                        type Response = super::MsgUpdateMasterMinterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateMasterMinter>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_master_minter(&inner, request).await
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
                        let method = UpdateMasterMinterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/UpdatePauser" => {
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
                "/circle.fiattokenfactory.v1.Msg/UpdateBlacklister" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBlacklisterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateBlacklister> for UpdateBlacklisterSvc<T> {
                        type Response = super::MsgUpdateBlacklisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateBlacklister>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_blacklister(&inner, request).await
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
                        let method = UpdateBlacklisterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/UpdateOwner" => {
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
                "/circle.fiattokenfactory.v1.Msg/AcceptOwner" => {
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
                "/circle.fiattokenfactory.v1.Msg/ConfigureMinter" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureMinterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgConfigureMinter> for ConfigureMinterSvc<T> {
                        type Response = super::MsgConfigureMinterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgConfigureMinter>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::configure_minter(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConfigureMinterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/RemoveMinter" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMinterSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRemoveMinter> for RemoveMinterSvc<T> {
                        type Response = super::MsgRemoveMinterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRemoveMinter>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as Msg>::remove_minter(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveMinterSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Mint" => {
                    #[allow(non_camel_case_types)]
                    struct MintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMint> for MintSvc<T> {
                        type Response = super::MsgMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMint>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::mint(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MintSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Burn" => {
                    #[allow(non_camel_case_types)]
                    struct BurnSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBurn> for BurnSvc<T> {
                        type Response = super::MsgBurnResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBurn>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::burn(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BurnSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Blacklist" => {
                    #[allow(non_camel_case_types)]
                    struct BlacklistSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBlacklist> for BlacklistSvc<T> {
                        type Response = super::MsgBlacklistResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBlacklist>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::blacklist(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BlacklistSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Unblacklist" => {
                    #[allow(non_camel_case_types)]
                    struct UnblacklistSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnblacklist> for UnblacklistSvc<T> {
                        type Response = super::MsgUnblacklistResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnblacklist>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::unblacklist(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnblacklistSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Pause" => {
                    #[allow(non_camel_case_types)]
                    struct PauseSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPause> for PauseSvc<T> {
                        type Response = super::MsgPauseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPause>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::pause(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PauseSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/Unpause" => {
                    #[allow(non_camel_case_types)]
                    struct UnpauseSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUnpause> for UnpauseSvc<T> {
                        type Response = super::MsgUnpauseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUnpause>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as Msg>::unpause(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnpauseSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/ConfigureMinterController" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureMinterControllerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgConfigureMinterController>
                        for ConfigureMinterControllerSvc<T>
                    {
                        type Response = super::MsgConfigureMinterControllerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgConfigureMinterController>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::configure_minter_controller(&inner, request).await
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
                        let method = ConfigureMinterControllerSvc(inner);
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
                "/circle.fiattokenfactory.v1.Msg/RemoveMinterController" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMinterControllerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRemoveMinterController>
                        for RemoveMinterControllerSvc<T>
                    {
                        type Response = super::MsgRemoveMinterControllerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRemoveMinterController>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::remove_minter_controller(&inner, request).await
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
                        let method = RemoveMinterControllerSvc(inner);
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
        const NAME: &'static str = "circle.fiattokenfactory.v1.Msg";
    }
}
