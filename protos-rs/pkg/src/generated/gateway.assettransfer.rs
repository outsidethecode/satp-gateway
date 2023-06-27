#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializationRequest {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_context_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializationResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_context_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub hash_transfer_init_claims: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub timestamp: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializationDenied {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferInitializationClaim {
    #[prost(string, tag = "1")]
    pub asset_asset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_profile_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub verified_originator_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub verified_beneficiary_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub originator_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub beneficiary_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub sender_gateway_network_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub recipient_gateway_network_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub client_identity_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub server_identity_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub sender_gateway_owner_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub receiver_gateway_owner_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod asset_transfer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// definitions of all messages used in the gateway
    #[derive(Debug, Clone)]
    pub struct AssetTransferClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AssetTransferClient<tonic::transport::Channel> {
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
    impl<T> AssetTransferClient<T>
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
        ) -> AssetTransferClient<InterceptedService<T, F>>
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
            AssetTransferClient::new(InterceptedService::new(inner, interceptor))
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
        /// the requesting gateway sends an TransferInitiationRequest request to the remote gateway
        pub async fn transfer_initiation_request(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
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
                "/gateway.assettransfer.AssetTransfer/TransferInitiationRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// the remote gateway sends an TransferInitiationResponse request to the requesting gateway
        pub async fn transfer_initiation_response(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializationResponse>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
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
                "/gateway.assettransfer.AssetTransfer/TransferInitiationResponse",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// the remote gateway sends an TransferInitiationDenied request to the requesting gateway
        pub async fn transfer_initiation_denied(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializationDenied>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
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
                "/gateway.assettransfer.AssetTransfer/TransferInitiationDenied",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod asset_transfer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AssetTransferServer.
    #[async_trait]
    pub trait AssetTransfer: Send + Sync + 'static {
        /// the requesting gateway sends an TransferInitiationRequest request to the remote gateway
        async fn transfer_initiation_request(
            &self,
            request: tonic::Request<super::InitializationRequest>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
            tonic::Status,
        >;
        /// the remote gateway sends an TransferInitiationResponse request to the requesting gateway
        async fn transfer_initiation_response(
            &self,
            request: tonic::Request<super::InitializationResponse>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
            tonic::Status,
        >;
        /// the remote gateway sends an TransferInitiationDenied request to the requesting gateway
        async fn transfer_initiation_denied(
            &self,
            request: tonic::Request<super::InitializationDenied>,
        ) -> Result<
            tonic::Response<super::super::super::common::ack::Ack>,
            tonic::Status,
        >;
    }
    /// definitions of all messages used in the gateway
    #[derive(Debug)]
    pub struct AssetTransferServer<T: AssetTransfer> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AssetTransfer> AssetTransferServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AssetTransferServer<T>
    where
        T: AssetTransfer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/gateway.assettransfer.AssetTransfer/TransferInitiationRequest" => {
                    #[allow(non_camel_case_types)]
                    struct TransferInitiationRequestSvc<T: AssetTransfer>(pub Arc<T>);
                    impl<
                        T: AssetTransfer,
                    > tonic::server::UnaryService<super::InitializationRequest>
                    for TransferInitiationRequestSvc<T> {
                        type Response = super::super::super::common::ack::Ack;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_initiation_request(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferInitiationRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.assettransfer.AssetTransfer/TransferInitiationResponse" => {
                    #[allow(non_camel_case_types)]
                    struct TransferInitiationResponseSvc<T: AssetTransfer>(pub Arc<T>);
                    impl<
                        T: AssetTransfer,
                    > tonic::server::UnaryService<super::InitializationResponse>
                    for TransferInitiationResponseSvc<T> {
                        type Response = super::super::super::common::ack::Ack;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializationResponse>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_initiation_response(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferInitiationResponseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.assettransfer.AssetTransfer/TransferInitiationDenied" => {
                    #[allow(non_camel_case_types)]
                    struct TransferInitiationDeniedSvc<T: AssetTransfer>(pub Arc<T>);
                    impl<
                        T: AssetTransfer,
                    > tonic::server::UnaryService<super::InitializationDenied>
                    for TransferInitiationDeniedSvc<T> {
                        type Response = super::super::super::common::ack::Ack;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializationDenied>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_initiation_denied(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferInitiationDeniedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AssetTransfer> Clone for AssetTransferServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AssetTransfer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AssetTransfer> tonic::server::NamedService for AssetTransferServer<T> {
        const NAME: &'static str = "gateway.assettransfer.AssetTransfer";
    }
}
