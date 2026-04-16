// @generated
/// Generated client implementations.
pub mod auth_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
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
    impl<T> AuthServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> AuthServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AuthServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<super::LoginResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/Login",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("diag.v1.AuthService", "Login"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn logout(
            &mut self,
            request: impl tonic::IntoRequest<super::LogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::LogoutResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/Logout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "Logout"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_me(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeRequest>,
        ) -> std::result::Result<tonic::Response<super::GetMeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/GetMe",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("diag.v1.AuthService", "GetMe"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/Register",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "Register"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_csrf_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCsrfTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCsrfTokenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/GetCSRFToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "GetCSRFToken"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn forgot_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ForgotPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ForgotPasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/ForgotPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "ForgotPassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetPasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/ResetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "ResetPassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_email(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyEmailResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/VerifyEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "VerifyEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_verification(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendVerificationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/ResendVerification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "ResendVerification"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProvidersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/ListProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "ListProviders"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlink_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlinkProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkProviderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.AuthService/UnlinkProvider",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.AuthService", "UnlinkProvider"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthServiceServer.
    #[async_trait]
    pub trait AuthService: std::marker::Send + std::marker::Sync + 'static {
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<super::LoginResponse>, tonic::Status>;
        async fn logout(
            &self,
            request: tonic::Request<super::LogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::LogoutResponse>, tonic::Status>;
        async fn get_me(
            &self,
            request: tonic::Request<super::GetMeRequest>,
        ) -> std::result::Result<tonic::Response<super::GetMeResponse>, tonic::Status>;
        async fn register(
            &self,
            request: tonic::Request<super::RegisterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterResponse>,
            tonic::Status,
        >;
        async fn get_csrf_token(
            &self,
            request: tonic::Request<super::GetCsrfTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCsrfTokenResponse>,
            tonic::Status,
        >;
        async fn forgot_password(
            &self,
            request: tonic::Request<super::ForgotPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ForgotPasswordResponse>,
            tonic::Status,
        >;
        async fn reset_password(
            &self,
            request: tonic::Request<super::ResetPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResetPasswordResponse>,
            tonic::Status,
        >;
        async fn verify_email(
            &self,
            request: tonic::Request<super::VerifyEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyEmailResponse>,
            tonic::Status,
        >;
        async fn resend_verification(
            &self,
            request: tonic::Request<super::ResendVerificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendVerificationResponse>,
            tonic::Status,
        >;
        async fn list_providers(
            &self,
            request: tonic::Request<super::ListProvidersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProvidersResponse>,
            tonic::Status,
        >;
        async fn unlink_provider(
            &self,
            request: tonic::Request<super::UnlinkProviderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnlinkProviderResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AuthServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AuthServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServiceServer<T>
    where
        T: AuthService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.AuthService/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::LoginRequest>
                    for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::login(&inner, request).await
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
                        let method = LoginSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/Logout" => {
                    #[allow(non_camel_case_types)]
                    struct LogoutSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::LogoutRequest>
                    for LogoutSvc<T> {
                        type Response = super::LogoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogoutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::logout(&inner, request).await
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
                        let method = LogoutSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/GetMe" => {
                    #[allow(non_camel_case_types)]
                    struct GetMeSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::GetMeRequest>
                    for GetMeSvc<T> {
                        type Response = super::GetMeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::get_me(&inner, request).await
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
                        let method = GetMeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::RegisterRequest>
                    for RegisterSvc<T> {
                        type Response = super::RegisterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::register(&inner, request).await
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
                        let method = RegisterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/GetCSRFToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetCSRFTokenSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::GetCsrfTokenRequest>
                    for GetCSRFTokenSvc<T> {
                        type Response = super::GetCsrfTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCsrfTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::get_csrf_token(&inner, request).await
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
                        let method = GetCSRFTokenSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/ForgotPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ForgotPasswordSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ForgotPasswordRequest>
                    for ForgotPasswordSvc<T> {
                        type Response = super::ForgotPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForgotPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::forgot_password(&inner, request).await
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
                        let method = ForgotPasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/ResetPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ResetPasswordSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ResetPasswordRequest>
                    for ResetPasswordSvc<T> {
                        type Response = super::ResetPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::reset_password(&inner, request).await
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
                        let method = ResetPasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/VerifyEmail" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyEmailSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::VerifyEmailRequest>
                    for VerifyEmailSvc<T> {
                        type Response = super::VerifyEmailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyEmailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::verify_email(&inner, request).await
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
                        let method = VerifyEmailSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/ResendVerification" => {
                    #[allow(non_camel_case_types)]
                    struct ResendVerificationSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ResendVerificationRequest>
                    for ResendVerificationSvc<T> {
                        type Response = super::ResendVerificationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResendVerificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::resend_verification(&inner, request)
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
                        let method = ResendVerificationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/ListProviders" => {
                    #[allow(non_camel_case_types)]
                    struct ListProvidersSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::ListProvidersRequest>
                    for ListProvidersSvc<T> {
                        type Response = super::ListProvidersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProvidersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::list_providers(&inner, request).await
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
                        let method = ListProvidersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.AuthService/UnlinkProvider" => {
                    #[allow(non_camel_case_types)]
                    struct UnlinkProviderSvc<T: AuthService>(pub Arc<T>);
                    impl<
                        T: AuthService,
                    > tonic::server::UnaryService<super::UnlinkProviderRequest>
                    for UnlinkProviderSvc<T> {
                        type Response = super::UnlinkProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlinkProviderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthService>::unlink_provider(&inner, request).await
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
                        let method = UnlinkProviderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AuthServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.AuthService";
    impl<T> tonic::server::NamedService for AuthServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod billing_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BillingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BillingServiceClient<tonic::transport::Channel> {
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
    impl<T> BillingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> BillingServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            BillingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBillingStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBillingStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/GetStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "GetStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::StartTrialRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartTrialResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/StartTrial",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "StartTrial"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn record_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/RecordSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "RecordSubscription"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/CancelSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "CancelSubscription"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_creem_checkout(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCreemCheckoutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCreemCheckoutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/CreateCreemCheckout",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.BillingService", "CreateCreemCheckout"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_creem_portal(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCreemPortalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCreemPortalResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/GetCreemPortal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "GetCreemPortal"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_creem_seats(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCreemSeatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCreemSeatsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.BillingService/UpdateCreemSeats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.BillingService", "UpdateCreemSeats"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod billing_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BillingServiceServer.
    #[async_trait]
    pub trait BillingService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_status(
            &self,
            request: tonic::Request<super::GetBillingStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBillingStatusResponse>,
            tonic::Status,
        >;
        async fn start_trial(
            &self,
            request: tonic::Request<super::StartTrialRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartTrialResponse>,
            tonic::Status,
        >;
        async fn record_subscription(
            &self,
            request: tonic::Request<super::RecordSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecordSubscriptionResponse>,
            tonic::Status,
        >;
        async fn cancel_subscription(
            &self,
            request: tonic::Request<super::CancelSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelSubscriptionResponse>,
            tonic::Status,
        >;
        async fn create_creem_checkout(
            &self,
            request: tonic::Request<super::CreateCreemCheckoutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCreemCheckoutResponse>,
            tonic::Status,
        >;
        async fn get_creem_portal(
            &self,
            request: tonic::Request<super::GetCreemPortalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCreemPortalResponse>,
            tonic::Status,
        >;
        async fn update_creem_seats(
            &self,
            request: tonic::Request<super::UpdateCreemSeatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCreemSeatsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BillingServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> BillingServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BillingServiceServer<T>
    where
        T: BillingService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.BillingService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::GetBillingStatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::GetBillingStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBillingStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::get_status(&inner, request).await
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
                        let method = GetStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/StartTrial" => {
                    #[allow(non_camel_case_types)]
                    struct StartTrialSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::StartTrialRequest>
                    for StartTrialSvc<T> {
                        type Response = super::StartTrialResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartTrialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::start_trial(&inner, request).await
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
                        let method = StartTrialSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/RecordSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSubscriptionSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::RecordSubscriptionRequest>
                    for RecordSubscriptionSvc<T> {
                        type Response = super::RecordSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::record_subscription(&inner, request)
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
                        let method = RecordSubscriptionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/CancelSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSubscriptionSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::CancelSubscriptionRequest>
                    for CancelSubscriptionSvc<T> {
                        type Response = super::CancelSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::cancel_subscription(&inner, request)
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
                        let method = CancelSubscriptionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/CreateCreemCheckout" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCreemCheckoutSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::CreateCreemCheckoutRequest>
                    for CreateCreemCheckoutSvc<T> {
                        type Response = super::CreateCreemCheckoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCreemCheckoutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::create_creem_checkout(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateCreemCheckoutSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/GetCreemPortal" => {
                    #[allow(non_camel_case_types)]
                    struct GetCreemPortalSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::GetCreemPortalRequest>
                    for GetCreemPortalSvc<T> {
                        type Response = super::GetCreemPortalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCreemPortalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::get_creem_portal(&inner, request)
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
                        let method = GetCreemPortalSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.BillingService/UpdateCreemSeats" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCreemSeatsSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::UpdateCreemSeatsRequest>
                    for UpdateCreemSeatsSvc<T> {
                        type Response = super::UpdateCreemSeatsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCreemSeatsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BillingService>::update_creem_seats(&inner, request)
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
                        let method = UpdateCreemSeatsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for BillingServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.BillingService";
    impl<T> tonic::server::NamedService for BillingServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod dependency_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DependencyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DependencyServiceClient<tonic::transport::Channel> {
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
    impl<T> DependencyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> DependencyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DependencyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_dependencies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDependenciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.DependencyService/ListDependencies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.DependencyService", "ListDependencies"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dependency_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DependencyServiceServer.
    #[async_trait]
    pub trait DependencyService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_dependencies(
            &self,
            request: tonic::Request<super::ListDependenciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDependenciesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DependencyServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DependencyServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DependencyServiceServer<T>
    where
        T: DependencyService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.DependencyService/ListDependencies" => {
                    #[allow(non_camel_case_types)]
                    struct ListDependenciesSvc<T: DependencyService>(pub Arc<T>);
                    impl<
                        T: DependencyService,
                    > tonic::server::UnaryService<super::ListDependenciesRequest>
                    for ListDependenciesSvc<T> {
                        type Response = super::ListDependenciesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDependenciesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DependencyService>::list_dependencies(&inner, request)
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
                        let method = ListDependenciesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for DependencyServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.DependencyService";
    impl<T> tonic::server::NamedService for DependencyServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod device_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DeviceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceServiceClient<tonic::transport::Channel> {
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
    impl<T> DeviceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> DeviceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            DeviceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn authorize(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceAuthorizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceAuthorizeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.DeviceService/Authorize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.DeviceService", "Authorize"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn poll_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DevicePollTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DevicePollTokenResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.DeviceService/PollToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.DeviceService", "PollToken"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_code_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCodeInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCodeInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.DeviceService/GetCodeInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.DeviceService", "GetCodeInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceConfirmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceConfirmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.DeviceService/Confirm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.DeviceService", "Confirm"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod device_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DeviceServiceServer.
    #[async_trait]
    pub trait DeviceService: std::marker::Send + std::marker::Sync + 'static {
        async fn authorize(
            &self,
            request: tonic::Request<super::DeviceAuthorizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceAuthorizeResponse>,
            tonic::Status,
        >;
        async fn poll_token(
            &self,
            request: tonic::Request<super::DevicePollTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DevicePollTokenResponse>,
            tonic::Status,
        >;
        async fn get_code_info(
            &self,
            request: tonic::Request<super::GetCodeInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCodeInfoResponse>,
            tonic::Status,
        >;
        async fn confirm(
            &self,
            request: tonic::Request<super::DeviceConfirmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceConfirmResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DeviceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DeviceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DeviceServiceServer<T>
    where
        T: DeviceService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.DeviceService/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceAuthorizeRequest>
                    for AuthorizeSvc<T> {
                        type Response = super::DeviceAuthorizeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceAuthorizeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::authorize(&inner, request).await
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
                        let method = AuthorizeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.DeviceService/PollToken" => {
                    #[allow(non_camel_case_types)]
                    struct PollTokenSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DevicePollTokenRequest>
                    for PollTokenSvc<T> {
                        type Response = super::DevicePollTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DevicePollTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::poll_token(&inner, request).await
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
                        let method = PollTokenSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.DeviceService/GetCodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetCodeInfoSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::GetCodeInfoRequest>
                    for GetCodeInfoSvc<T> {
                        type Response = super::GetCodeInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCodeInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::get_code_info(&inner, request).await
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
                        let method = GetCodeInfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.DeviceService/Confirm" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceConfirmRequest>
                    for ConfirmSvc<T> {
                        type Response = super::DeviceConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceConfirmRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DeviceService>::confirm(&inner, request).await
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
                        let method = ConfirmSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for DeviceServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.DeviceService";
    impl<T> tonic::server::NamedService for DeviceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod explore_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ExploreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExploreServiceClient<tonic::transport::Channel> {
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
    impl<T> ExploreServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ExploreServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ExploreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn share_view(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShareViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.ExploreService/ShareView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.ExploreService", "ShareView"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod explore_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExploreServiceServer.
    #[async_trait]
    pub trait ExploreService: std::marker::Send + std::marker::Sync + 'static {
        async fn share_view(
            &self,
            request: tonic::Request<super::ShareViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShareViewResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ExploreServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ExploreServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExploreServiceServer<T>
    where
        T: ExploreService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.ExploreService/ShareView" => {
                    #[allow(non_camel_case_types)]
                    struct ShareViewSvc<T: ExploreService>(pub Arc<T>);
                    impl<
                        T: ExploreService,
                    > tonic::server::UnaryService<super::ShareViewRequest>
                    for ShareViewSvc<T> {
                        type Response = super::ShareViewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShareViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExploreService>::share_view(&inner, request).await
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
                        let method = ShareViewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ExploreServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.ExploreService";
    impl<T> tonic::server::NamedService for ExploreServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod workspace_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WorkspaceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkspaceServiceClient<tonic::transport::Channel> {
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
    impl<T> WorkspaceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> WorkspaceServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            WorkspaceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_view(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreateView"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_workspace_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyPlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApplyPlanResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ApplyWorkspacePlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceService", "ApplyWorkspacePlan"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn export_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ExportWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ExportWorkspace"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_view(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "DeleteView"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_element(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteElement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "DeleteElement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteConnectorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "DeleteConnector"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_element_navigation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteElementNavigationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementNavigationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteElementNavigation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "diag.v1.WorkspaceService",
                        "DeleteElementNavigation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_views(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViewsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListViews",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListViews"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/GetView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "GetView"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_view(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateViewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/UpdateView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "UpdateView"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_view_level(
            &mut self,
            request: impl tonic::IntoRequest<super::SetViewLevelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetViewLevelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/SetViewLevel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "SetViewLevel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWorkspaceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/GetWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "GetWorkspace"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_incoming_element_navigations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListIncomingElementNavigationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListIncomingElementNavigationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListIncomingElementNavigations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "diag.v1.WorkspaceService",
                        "ListIncomingElementNavigations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_elements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListElementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListElements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListElements"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_element(
            &mut self,
            request: impl tonic::IntoRequest<super::GetElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetElementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/GetElement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "GetElement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_element(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateElementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateElement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreateElement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_element(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateElementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/UpdateElement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "UpdateElement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_element_placements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListElementPlacementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementPlacementsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListElementPlacements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceService", "ListElementPlacements"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_placements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPlacementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPlacementsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListPlacements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListPlacements"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_placement(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePlacementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePlacementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreatePlacement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreatePlacement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_placement_position(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlacementPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePlacementPositionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/UpdatePlacementPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "diag.v1.WorkspaceService",
                        "UpdatePlacementPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_placement(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePlacementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePlacementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeletePlacement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "DeletePlacement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListConnectors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListConnectors"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConnectorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreateConnector"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConnectorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/UpdateConnector",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "UpdateConnector"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_element_navigations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListElementNavigationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementNavigationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListElementNavigations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceService", "ListElementNavigations"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_element_navigation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateElementNavigationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateElementNavigationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateElementNavigation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "diag.v1.WorkspaceService",
                        "CreateElementNavigation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_element_navigation_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteElementNavigationByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementNavigationByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteElementNavigationById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "diag.v1.WorkspaceService",
                        "DeleteElementNavigationById",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_view_layers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViewLayersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewLayersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListViewLayers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListViewLayers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_view_layer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateViewLayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateViewLayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreateViewLayer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_view_layer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateViewLayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/UpdateViewLayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "UpdateViewLayer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_view_layer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteViewLayerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/DeleteViewLayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "DeleteViewLayer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_threads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListThreadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListThreadsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListThreads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListThreads"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateThreadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/CreateThread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "CreateThread"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_comment(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCommentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCommentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/AddComment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "AddComment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveThreadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ResolveThread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ResolveThread"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_reactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReactionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ListReactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ListReactions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn toggle_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ToggleReactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceService/ToggleReaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceService", "ToggleReaction"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod workspace_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WorkspaceServiceServer.
    #[async_trait]
    pub trait WorkspaceService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_view(
            &self,
            request: tonic::Request<super::CreateViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateViewResponse>,
            tonic::Status,
        >;
        async fn apply_workspace_plan(
            &self,
            request: tonic::Request<super::ApplyPlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApplyPlanResponse>,
            tonic::Status,
        >;
        async fn export_workspace(
            &self,
            request: tonic::Request<super::ExportOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExportOrganizationResponse>,
            tonic::Status,
        >;
        async fn delete_view(
            &self,
            request: tonic::Request<super::DeleteViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteViewResponse>,
            tonic::Status,
        >;
        async fn delete_element(
            &self,
            request: tonic::Request<super::DeleteElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementResponse>,
            tonic::Status,
        >;
        async fn delete_connector(
            &self,
            request: tonic::Request<super::DeleteConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteConnectorResponse>,
            tonic::Status,
        >;
        async fn delete_element_navigation(
            &self,
            request: tonic::Request<super::DeleteElementNavigationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementNavigationResponse>,
            tonic::Status,
        >;
        async fn list_views(
            &self,
            request: tonic::Request<super::ListViewsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewsResponse>,
            tonic::Status,
        >;
        async fn get_view(
            &self,
            request: tonic::Request<super::GetViewRequest>,
        ) -> std::result::Result<tonic::Response<super::GetViewResponse>, tonic::Status>;
        async fn update_view(
            &self,
            request: tonic::Request<super::UpdateViewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateViewResponse>,
            tonic::Status,
        >;
        async fn set_view_level(
            &self,
            request: tonic::Request<super::SetViewLevelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetViewLevelResponse>,
            tonic::Status,
        >;
        async fn get_workspace(
            &self,
            request: tonic::Request<super::GetWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWorkspaceResponse>,
            tonic::Status,
        >;
        async fn list_incoming_element_navigations(
            &self,
            request: tonic::Request<super::ListIncomingElementNavigationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListIncomingElementNavigationsResponse>,
            tonic::Status,
        >;
        async fn list_elements(
            &self,
            request: tonic::Request<super::ListElementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementsResponse>,
            tonic::Status,
        >;
        async fn get_element(
            &self,
            request: tonic::Request<super::GetElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetElementResponse>,
            tonic::Status,
        >;
        async fn create_element(
            &self,
            request: tonic::Request<super::CreateElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateElementResponse>,
            tonic::Status,
        >;
        async fn update_element(
            &self,
            request: tonic::Request<super::UpdateElementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateElementResponse>,
            tonic::Status,
        >;
        async fn list_element_placements(
            &self,
            request: tonic::Request<super::ListElementPlacementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementPlacementsResponse>,
            tonic::Status,
        >;
        async fn list_placements(
            &self,
            request: tonic::Request<super::ListPlacementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPlacementsResponse>,
            tonic::Status,
        >;
        async fn create_placement(
            &self,
            request: tonic::Request<super::CreatePlacementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePlacementResponse>,
            tonic::Status,
        >;
        async fn update_placement_position(
            &self,
            request: tonic::Request<super::UpdatePlacementPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePlacementPositionResponse>,
            tonic::Status,
        >;
        async fn delete_placement(
            &self,
            request: tonic::Request<super::DeletePlacementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePlacementResponse>,
            tonic::Status,
        >;
        async fn list_connectors(
            &self,
            request: tonic::Request<super::ListConnectorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectorsResponse>,
            tonic::Status,
        >;
        async fn create_connector(
            &self,
            request: tonic::Request<super::CreateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConnectorResponse>,
            tonic::Status,
        >;
        async fn update_connector(
            &self,
            request: tonic::Request<super::UpdateConnectorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConnectorResponse>,
            tonic::Status,
        >;
        async fn list_element_navigations(
            &self,
            request: tonic::Request<super::ListElementNavigationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListElementNavigationsResponse>,
            tonic::Status,
        >;
        async fn create_element_navigation(
            &self,
            request: tonic::Request<super::CreateElementNavigationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateElementNavigationResponse>,
            tonic::Status,
        >;
        async fn delete_element_navigation_by_id(
            &self,
            request: tonic::Request<super::DeleteElementNavigationByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteElementNavigationByIdResponse>,
            tonic::Status,
        >;
        async fn list_view_layers(
            &self,
            request: tonic::Request<super::ListViewLayersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListViewLayersResponse>,
            tonic::Status,
        >;
        async fn create_view_layer(
            &self,
            request: tonic::Request<super::CreateViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateViewLayerResponse>,
            tonic::Status,
        >;
        async fn update_view_layer(
            &self,
            request: tonic::Request<super::UpdateViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateViewLayerResponse>,
            tonic::Status,
        >;
        async fn delete_view_layer(
            &self,
            request: tonic::Request<super::DeleteViewLayerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteViewLayerResponse>,
            tonic::Status,
        >;
        async fn list_threads(
            &self,
            request: tonic::Request<super::ListThreadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListThreadsResponse>,
            tonic::Status,
        >;
        async fn create_thread(
            &self,
            request: tonic::Request<super::CreateThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateThreadResponse>,
            tonic::Status,
        >;
        async fn add_comment(
            &self,
            request: tonic::Request<super::AddCommentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCommentResponse>,
            tonic::Status,
        >;
        async fn resolve_thread(
            &self,
            request: tonic::Request<super::ResolveThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveThreadResponse>,
            tonic::Status,
        >;
        async fn list_reactions(
            &self,
            request: tonic::Request<super::ListReactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReactionsResponse>,
            tonic::Status,
        >;
        async fn toggle_reaction(
            &self,
            request: tonic::Request<super::ToggleReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ToggleReactionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WorkspaceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> WorkspaceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkspaceServiceServer<T>
    where
        T: WorkspaceService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.WorkspaceService/CreateView" => {
                    #[allow(non_camel_case_types)]
                    struct CreateViewSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateViewRequest>
                    for CreateViewSvc<T> {
                        type Response = super::CreateViewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_view(&inner, request).await
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
                        let method = CreateViewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ApplyWorkspacePlan" => {
                    #[allow(non_camel_case_types)]
                    struct ApplyWorkspacePlanSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ApplyPlanRequest>
                    for ApplyWorkspacePlanSvc<T> {
                        type Response = super::ApplyPlanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyPlanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::apply_workspace_plan(
                                        &inner,
                                        request,
                                    )
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
                        let method = ApplyWorkspacePlanSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ExportWorkspace" => {
                    #[allow(non_camel_case_types)]
                    struct ExportWorkspaceSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ExportOrganizationRequest>
                    for ExportWorkspaceSvc<T> {
                        type Response = super::ExportOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::export_workspace(&inner, request)
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
                        let method = ExportWorkspaceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteView" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteViewSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteViewRequest>
                    for DeleteViewSvc<T> {
                        type Response = super::DeleteViewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_view(&inner, request).await
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
                        let method = DeleteViewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteElement" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteElementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteElementRequest>
                    for DeleteElementSvc<T> {
                        type Response = super::DeleteElementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteElementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_element(&inner, request)
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
                        let method = DeleteElementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteConnector" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteConnectorSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteConnectorRequest>
                    for DeleteConnectorSvc<T> {
                        type Response = super::DeleteConnectorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_connector(&inner, request)
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
                        let method = DeleteConnectorSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteElementNavigation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteElementNavigationSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteElementNavigationRequest>
                    for DeleteElementNavigationSvc<T> {
                        type Response = super::DeleteElementNavigationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteElementNavigationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_element_navigation(
                                        &inner,
                                        request,
                                    )
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
                        let method = DeleteElementNavigationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListViews" => {
                    #[allow(non_camel_case_types)]
                    struct ListViewsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListViewsRequest>
                    for ListViewsSvc<T> {
                        type Response = super::ListViewsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListViewsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_views(&inner, request).await
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
                        let method = ListViewsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/GetView" => {
                    #[allow(non_camel_case_types)]
                    struct GetViewSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::GetViewRequest>
                    for GetViewSvc<T> {
                        type Response = super::GetViewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::get_view(&inner, request).await
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
                        let method = GetViewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/UpdateView" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateViewSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::UpdateViewRequest>
                    for UpdateViewSvc<T> {
                        type Response = super::UpdateViewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::update_view(&inner, request).await
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
                        let method = UpdateViewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/SetViewLevel" => {
                    #[allow(non_camel_case_types)]
                    struct SetViewLevelSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::SetViewLevelRequest>
                    for SetViewLevelSvc<T> {
                        type Response = super::SetViewLevelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetViewLevelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::set_view_level(&inner, request)
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
                        let method = SetViewLevelSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/GetWorkspace" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkspaceSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::GetWorkspaceRequest>
                    for GetWorkspaceSvc<T> {
                        type Response = super::GetWorkspaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkspaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::get_workspace(&inner, request)
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
                        let method = GetWorkspaceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListIncomingElementNavigations" => {
                    #[allow(non_camel_case_types)]
                    struct ListIncomingElementNavigationsSvc<T: WorkspaceService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<
                        super::ListIncomingElementNavigationsRequest,
                    > for ListIncomingElementNavigationsSvc<T> {
                        type Response = super::ListIncomingElementNavigationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListIncomingElementNavigationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_incoming_element_navigations(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListIncomingElementNavigationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListElements" => {
                    #[allow(non_camel_case_types)]
                    struct ListElementsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListElementsRequest>
                    for ListElementsSvc<T> {
                        type Response = super::ListElementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListElementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_elements(&inner, request)
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
                        let method = ListElementsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/GetElement" => {
                    #[allow(non_camel_case_types)]
                    struct GetElementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::GetElementRequest>
                    for GetElementSvc<T> {
                        type Response = super::GetElementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetElementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::get_element(&inner, request).await
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
                        let method = GetElementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreateElement" => {
                    #[allow(non_camel_case_types)]
                    struct CreateElementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateElementRequest>
                    for CreateElementSvc<T> {
                        type Response = super::CreateElementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateElementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_element(&inner, request)
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
                        let method = CreateElementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/UpdateElement" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateElementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::UpdateElementRequest>
                    for UpdateElementSvc<T> {
                        type Response = super::UpdateElementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateElementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::update_element(&inner, request)
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
                        let method = UpdateElementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListElementPlacements" => {
                    #[allow(non_camel_case_types)]
                    struct ListElementPlacementsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListElementPlacementsRequest>
                    for ListElementPlacementsSvc<T> {
                        type Response = super::ListElementPlacementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListElementPlacementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_element_placements(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListElementPlacementsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListPlacements" => {
                    #[allow(non_camel_case_types)]
                    struct ListPlacementsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListPlacementsRequest>
                    for ListPlacementsSvc<T> {
                        type Response = super::ListPlacementsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPlacementsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_placements(&inner, request)
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
                        let method = ListPlacementsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreatePlacement" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePlacementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreatePlacementRequest>
                    for CreatePlacementSvc<T> {
                        type Response = super::CreatePlacementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePlacementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_placement(&inner, request)
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
                        let method = CreatePlacementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/UpdatePlacementPosition" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePlacementPositionSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::UpdatePlacementPositionRequest>
                    for UpdatePlacementPositionSvc<T> {
                        type Response = super::UpdatePlacementPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdatePlacementPositionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::update_placement_position(
                                        &inner,
                                        request,
                                    )
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
                        let method = UpdatePlacementPositionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeletePlacement" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePlacementSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeletePlacementRequest>
                    for DeletePlacementSvc<T> {
                        type Response = super::DeletePlacementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePlacementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_placement(&inner, request)
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
                        let method = DeletePlacementSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListConnectors" => {
                    #[allow(non_camel_case_types)]
                    struct ListConnectorsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListConnectorsRequest>
                    for ListConnectorsSvc<T> {
                        type Response = super::ListConnectorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConnectorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_connectors(&inner, request)
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
                        let method = ListConnectorsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreateConnector" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConnectorSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateConnectorRequest>
                    for CreateConnectorSvc<T> {
                        type Response = super::CreateConnectorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_connector(&inner, request)
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
                        let method = CreateConnectorSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/UpdateConnector" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConnectorSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::UpdateConnectorRequest>
                    for UpdateConnectorSvc<T> {
                        type Response = super::UpdateConnectorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateConnectorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::update_connector(&inner, request)
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
                        let method = UpdateConnectorSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListElementNavigations" => {
                    #[allow(non_camel_case_types)]
                    struct ListElementNavigationsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListElementNavigationsRequest>
                    for ListElementNavigationsSvc<T> {
                        type Response = super::ListElementNavigationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListElementNavigationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_element_navigations(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListElementNavigationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreateElementNavigation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateElementNavigationSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateElementNavigationRequest>
                    for CreateElementNavigationSvc<T> {
                        type Response = super::CreateElementNavigationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateElementNavigationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_element_navigation(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateElementNavigationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteElementNavigationById" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteElementNavigationByIdSvc<T: WorkspaceService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<
                        super::DeleteElementNavigationByIdRequest,
                    > for DeleteElementNavigationByIdSvc<T> {
                        type Response = super::DeleteElementNavigationByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteElementNavigationByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_element_navigation_by_id(
                                        &inner,
                                        request,
                                    )
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
                        let method = DeleteElementNavigationByIdSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListViewLayers" => {
                    #[allow(non_camel_case_types)]
                    struct ListViewLayersSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListViewLayersRequest>
                    for ListViewLayersSvc<T> {
                        type Response = super::ListViewLayersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListViewLayersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_view_layers(&inner, request)
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
                        let method = ListViewLayersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreateViewLayer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateViewLayerSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateViewLayerRequest>
                    for CreateViewLayerSvc<T> {
                        type Response = super::CreateViewLayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateViewLayerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_view_layer(&inner, request)
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
                        let method = CreateViewLayerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/UpdateViewLayer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateViewLayerSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::UpdateViewLayerRequest>
                    for UpdateViewLayerSvc<T> {
                        type Response = super::UpdateViewLayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateViewLayerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::update_view_layer(&inner, request)
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
                        let method = UpdateViewLayerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/DeleteViewLayer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteViewLayerSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteViewLayerRequest>
                    for DeleteViewLayerSvc<T> {
                        type Response = super::DeleteViewLayerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteViewLayerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::delete_view_layer(&inner, request)
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
                        let method = DeleteViewLayerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListThreads" => {
                    #[allow(non_camel_case_types)]
                    struct ListThreadsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListThreadsRequest>
                    for ListThreadsSvc<T> {
                        type Response = super::ListThreadsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListThreadsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_threads(&inner, request).await
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
                        let method = ListThreadsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/CreateThread" => {
                    #[allow(non_camel_case_types)]
                    struct CreateThreadSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateThreadRequest>
                    for CreateThreadSvc<T> {
                        type Response = super::CreateThreadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateThreadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::create_thread(&inner, request)
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
                        let method = CreateThreadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/AddComment" => {
                    #[allow(non_camel_case_types)]
                    struct AddCommentSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::AddCommentRequest>
                    for AddCommentSvc<T> {
                        type Response = super::AddCommentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCommentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::add_comment(&inner, request).await
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
                        let method = AddCommentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ResolveThread" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveThreadSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ResolveThreadRequest>
                    for ResolveThreadSvc<T> {
                        type Response = super::ResolveThreadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveThreadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::resolve_thread(&inner, request)
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
                        let method = ResolveThreadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ListReactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListReactionsSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ListReactionsRequest>
                    for ListReactionsSvc<T> {
                        type Response = super::ListReactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReactionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::list_reactions(&inner, request)
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
                        let method = ListReactionsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceService/ToggleReaction" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleReactionSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ToggleReactionRequest>
                    for ToggleReactionSvc<T> {
                        type Response = super::ToggleReactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleReactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceService>::toggle_reaction(&inner, request)
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
                        let method = ToggleReactionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for WorkspaceServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.WorkspaceService";
    impl<T> tonic::server::NamedService for WorkspaceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod import_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ImportServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ImportServiceClient<tonic::transport::Channel> {
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
    impl<T> ImportServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ImportServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ImportServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn import_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportResourcesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.ImportService/ImportResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.ImportService", "ImportResources"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn parse_structurizr(
            &mut self,
            request: impl tonic::IntoRequest<super::ParseStructurizrRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ParseStructurizrResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.ImportService/ParseStructurizr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.ImportService", "ParseStructurizr"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod import_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ImportServiceServer.
    #[async_trait]
    pub trait ImportService: std::marker::Send + std::marker::Sync + 'static {
        async fn import_resources(
            &self,
            request: tonic::Request<super::ImportResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportResourcesResponse>,
            tonic::Status,
        >;
        async fn parse_structurizr(
            &self,
            request: tonic::Request<super::ParseStructurizrRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ParseStructurizrResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ImportServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ImportServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ImportServiceServer<T>
    where
        T: ImportService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.ImportService/ImportResources" => {
                    #[allow(non_camel_case_types)]
                    struct ImportResourcesSvc<T: ImportService>(pub Arc<T>);
                    impl<
                        T: ImportService,
                    > tonic::server::UnaryService<super::ImportResourcesRequest>
                    for ImportResourcesSvc<T> {
                        type Response = super::ImportResourcesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportResourcesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ImportService>::import_resources(&inner, request)
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
                        let method = ImportResourcesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.ImportService/ParseStructurizr" => {
                    #[allow(non_camel_case_types)]
                    struct ParseStructurizrSvc<T: ImportService>(pub Arc<T>);
                    impl<
                        T: ImportService,
                    > tonic::server::UnaryService<super::ParseStructurizrRequest>
                    for ParseStructurizrSvc<T> {
                        type Response = super::ParseStructurizrResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ParseStructurizrRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ImportService>::parse_structurizr(&inner, request)
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
                        let method = ParseStructurizrSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ImportServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.ImportService";
    impl<T> tonic::server::NamedService for ImportServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod notification_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NotificationServiceClient<tonic::transport::Channel> {
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
    impl<T> NotificationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> NotificationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            NotificationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.NotificationService/ListNotifications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.NotificationService", "ListNotifications"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn mark_read(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkNotificationReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkNotificationReadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.NotificationService/MarkRead",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.NotificationService", "MarkRead"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mark_all_read(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkAllNotificationsReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkAllNotificationsReadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.NotificationService/MarkAllRead",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.NotificationService", "MarkAllRead"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod notification_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NotificationServiceServer.
    #[async_trait]
    pub trait NotificationService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_notifications(
            &self,
            request: tonic::Request<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
            tonic::Status,
        >;
        async fn mark_read(
            &self,
            request: tonic::Request<super::MarkNotificationReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkNotificationReadResponse>,
            tonic::Status,
        >;
        async fn mark_all_read(
            &self,
            request: tonic::Request<super::MarkAllNotificationsReadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkAllNotificationsReadResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct NotificationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> NotificationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NotificationServiceServer<T>
    where
        T: NotificationService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.NotificationService/ListNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct ListNotificationsSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::ListNotificationsRequest>
                    for ListNotificationsSvc<T> {
                        type Response = super::ListNotificationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNotificationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::list_notifications(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListNotificationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.NotificationService/MarkRead" => {
                    #[allow(non_camel_case_types)]
                    struct MarkReadSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::MarkNotificationReadRequest>
                    for MarkReadSvc<T> {
                        type Response = super::MarkNotificationReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkNotificationReadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::mark_read(&inner, request).await
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
                        let method = MarkReadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.NotificationService/MarkAllRead" => {
                    #[allow(non_camel_case_types)]
                    struct MarkAllReadSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::MarkAllNotificationsReadRequest>
                    for MarkAllReadSvc<T> {
                        type Response = super::MarkAllNotificationsReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MarkAllNotificationsReadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::mark_all_read(&inner, request)
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
                        let method = MarkAllReadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for NotificationServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.NotificationService";
    impl<T> tonic::server::NamedService for NotificationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod org_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrgServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrgServiceClient<tonic::transport::Channel> {
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
    impl<T> OrgServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> OrgServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            OrgServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/GetOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "GetOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/UpdateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "UpdateOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_usage(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUsageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/GetUsage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "GetUsage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMembersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/ListMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "ListMembers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_member_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMemberRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMemberRoleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/UpdateMemberRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "UpdateMemberRole"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMemberResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/RemoveMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "RemoveMember"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInvitationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/CreateInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "CreateInvitation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_invitations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInvitationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInvitationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/ListInvitations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "ListInvitations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteInvitationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/DeleteInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "DeleteInvitation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendInvitationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/ResendInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "ResendInvitation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_api_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApiKeysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/ListAPIKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "ListAPIKeys"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/CreateAPIKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "CreateAPIKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn revoke_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeApiKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/RevokeAPIKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "RevokeAPIKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_tag_colors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagColorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTagColorsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/ListTagColors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "ListTagColors"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTagResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/UpdateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "UpdateTag"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn seed_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::SeedOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SeedOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.OrgService/SeedOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.OrgService", "SeedOrganization"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod org_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrgServiceServer.
    #[async_trait]
    pub trait OrgService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_organization(
            &self,
            request: tonic::Request<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
            tonic::Status,
        >;
        async fn update_organization(
            &self,
            request: tonic::Request<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
            tonic::Status,
        >;
        async fn get_usage(
            &self,
            request: tonic::Request<super::GetUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUsageResponse>,
            tonic::Status,
        >;
        async fn list_members(
            &self,
            request: tonic::Request<super::ListMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMembersResponse>,
            tonic::Status,
        >;
        async fn update_member_role(
            &self,
            request: tonic::Request<super::UpdateMemberRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMemberRoleResponse>,
            tonic::Status,
        >;
        async fn remove_member(
            &self,
            request: tonic::Request<super::RemoveMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMemberResponse>,
            tonic::Status,
        >;
        async fn create_invitation(
            &self,
            request: tonic::Request<super::CreateInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInvitationResponse>,
            tonic::Status,
        >;
        async fn list_invitations(
            &self,
            request: tonic::Request<super::ListInvitationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInvitationsResponse>,
            tonic::Status,
        >;
        async fn delete_invitation(
            &self,
            request: tonic::Request<super::DeleteInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteInvitationResponse>,
            tonic::Status,
        >;
        async fn resend_invitation(
            &self,
            request: tonic::Request<super::ResendInvitationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendInvitationResponse>,
            tonic::Status,
        >;
        async fn list_api_keys(
            &self,
            request: tonic::Request<super::ListApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApiKeysResponse>,
            tonic::Status,
        >;
        async fn create_api_key(
            &self,
            request: tonic::Request<super::CreateApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiKeyResponse>,
            tonic::Status,
        >;
        async fn revoke_api_key(
            &self,
            request: tonic::Request<super::RevokeApiKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeApiKeyResponse>,
            tonic::Status,
        >;
        async fn list_tag_colors(
            &self,
            request: tonic::Request<super::ListTagColorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTagColorsResponse>,
            tonic::Status,
        >;
        async fn update_tag(
            &self,
            request: tonic::Request<super::UpdateTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTagResponse>,
            tonic::Status,
        >;
        async fn seed_organization(
            &self,
            request: tonic::Request<super::SeedOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SeedOrganizationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OrgServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> OrgServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrgServiceServer<T>
    where
        T: OrgService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.OrgService/GetOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::GetOrganizationRequest>
                    for GetOrganizationSvc<T> {
                        type Response = super::GetOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::get_organization(&inner, request).await
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
                        let method = GetOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/UpdateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrganizationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::UpdateOrganizationRequest>
                    for UpdateOrganizationSvc<T> {
                        type Response = super::UpdateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::update_organization(&inner, request)
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
                        let method = UpdateOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/GetUsage" => {
                    #[allow(non_camel_case_types)]
                    struct GetUsageSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::GetUsageRequest>
                    for GetUsageSvc<T> {
                        type Response = super::GetUsageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUsageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::get_usage(&inner, request).await
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
                        let method = GetUsageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/ListMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListMembersSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::ListMembersRequest>
                    for ListMembersSvc<T> {
                        type Response = super::ListMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::list_members(&inner, request).await
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
                        let method = ListMembersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/UpdateMemberRole" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMemberRoleSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::UpdateMemberRoleRequest>
                    for UpdateMemberRoleSvc<T> {
                        type Response = super::UpdateMemberRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMemberRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::update_member_role(&inner, request).await
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
                        let method = UpdateMemberRoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/RemoveMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMemberSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::RemoveMemberRequest>
                    for RemoveMemberSvc<T> {
                        type Response = super::RemoveMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveMemberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::remove_member(&inner, request).await
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
                        let method = RemoveMemberSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/CreateInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInvitationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::CreateInvitationRequest>
                    for CreateInvitationSvc<T> {
                        type Response = super::CreateInvitationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::create_invitation(&inner, request).await
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
                        let method = CreateInvitationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/ListInvitations" => {
                    #[allow(non_camel_case_types)]
                    struct ListInvitationsSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::ListInvitationsRequest>
                    for ListInvitationsSvc<T> {
                        type Response = super::ListInvitationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInvitationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::list_invitations(&inner, request).await
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
                        let method = ListInvitationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/DeleteInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInvitationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::DeleteInvitationRequest>
                    for DeleteInvitationSvc<T> {
                        type Response = super::DeleteInvitationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::delete_invitation(&inner, request).await
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
                        let method = DeleteInvitationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/ResendInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct ResendInvitationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::ResendInvitationRequest>
                    for ResendInvitationSvc<T> {
                        type Response = super::ResendInvitationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResendInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::resend_invitation(&inner, request).await
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
                        let method = ResendInvitationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/ListAPIKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListAPIKeysSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::ListApiKeysRequest>
                    for ListAPIKeysSvc<T> {
                        type Response = super::ListApiKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListApiKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::list_api_keys(&inner, request).await
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
                        let method = ListAPIKeysSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/CreateAPIKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAPIKeySvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::CreateApiKeyRequest>
                    for CreateAPIKeySvc<T> {
                        type Response = super::CreateApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::create_api_key(&inner, request).await
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
                        let method = CreateAPIKeySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/RevokeAPIKey" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeAPIKeySvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::RevokeApiKeyRequest>
                    for RevokeAPIKeySvc<T> {
                        type Response = super::RevokeApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::revoke_api_key(&inner, request).await
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
                        let method = RevokeAPIKeySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/ListTagColors" => {
                    #[allow(non_camel_case_types)]
                    struct ListTagColorsSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::ListTagColorsRequest>
                    for ListTagColorsSvc<T> {
                        type Response = super::ListTagColorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTagColorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::list_tag_colors(&inner, request).await
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
                        let method = ListTagColorsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/UpdateTag" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTagSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::UpdateTagRequest>
                    for UpdateTagSvc<T> {
                        type Response = super::UpdateTagResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::update_tag(&inner, request).await
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
                        let method = UpdateTagSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.OrgService/SeedOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct SeedOrganizationSvc<T: OrgService>(pub Arc<T>);
                    impl<
                        T: OrgService,
                    > tonic::server::UnaryService<super::SeedOrganizationRequest>
                    for SeedOrganizationSvc<T> {
                        type Response = super::SeedOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SeedOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrgService>::seed_organization(&inner, request).await
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
                        let method = SeedOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for OrgServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.OrgService";
    impl<T> tonic::server::NamedService for OrgServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod user_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
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
    impl<T> UserServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> UserServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/ListOrganizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "ListOrganizations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn switch_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::SwitchOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SwitchOrganizationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/SwitchOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "SwitchOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPreferencesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/GetPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "GetPreferences"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePreferencesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/UpdatePreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "UpdatePreferences"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_preview(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePreviewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePreviewResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/DeletePreview",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "DeletePreview"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_org_ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferOrgOwnershipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferOrgOwnershipResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/TransferOrgOwnership",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "TransferOrgOwnership"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/DeleteAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "DeleteAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangePasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangePasswordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.UserService/ChangePassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.UserService", "ChangePassword"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_organizations(
            &self,
            request: tonic::Request<super::ListOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsResponse>,
            tonic::Status,
        >;
        async fn switch_organization(
            &self,
            request: tonic::Request<super::SwitchOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SwitchOrganizationResponse>,
            tonic::Status,
        >;
        async fn get_preferences(
            &self,
            request: tonic::Request<super::GetPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPreferencesResponse>,
            tonic::Status,
        >;
        async fn update_preferences(
            &self,
            request: tonic::Request<super::UpdatePreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdatePreferencesResponse>,
            tonic::Status,
        >;
        async fn delete_preview(
            &self,
            request: tonic::Request<super::DeletePreviewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeletePreviewResponse>,
            tonic::Status,
        >;
        async fn transfer_org_ownership(
            &self,
            request: tonic::Request<super::TransferOrgOwnershipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferOrgOwnershipResponse>,
            tonic::Status,
        >;
        async fn delete_account(
            &self,
            request: tonic::Request<super::DeleteAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAccountResponse>,
            tonic::Status,
        >;
        async fn change_password(
            &self,
            request: tonic::Request<super::ChangePasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangePasswordResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> UserServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.UserService/ListOrganizations" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrganizationsSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ListOrganizationsRequest>
                    for ListOrganizationsSvc<T> {
                        type Response = super::ListOrganizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOrganizationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::list_organizations(&inner, request)
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
                        let method = ListOrganizationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/SwitchOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct SwitchOrganizationSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::SwitchOrganizationRequest>
                    for SwitchOrganizationSvc<T> {
                        type Response = super::SwitchOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SwitchOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::switch_organization(&inner, request)
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
                        let method = SwitchOrganizationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/GetPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct GetPreferencesSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetPreferencesRequest>
                    for GetPreferencesSvc<T> {
                        type Response = super::GetPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPreferencesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::get_preferences(&inner, request).await
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
                        let method = GetPreferencesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/UpdatePreferences" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePreferencesSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::UpdatePreferencesRequest>
                    for UpdatePreferencesSvc<T> {
                        type Response = super::UpdatePreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePreferencesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::update_preferences(&inner, request)
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
                        let method = UpdatePreferencesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/DeletePreview" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePreviewSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeletePreviewRequest>
                    for DeletePreviewSvc<T> {
                        type Response = super::DeletePreviewResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePreviewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::delete_preview(&inner, request).await
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
                        let method = DeletePreviewSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/TransferOrgOwnership" => {
                    #[allow(non_camel_case_types)]
                    struct TransferOrgOwnershipSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::TransferOrgOwnershipRequest>
                    for TransferOrgOwnershipSvc<T> {
                        type Response = super::TransferOrgOwnershipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferOrgOwnershipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::transfer_org_ownership(&inner, request)
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
                        let method = TransferOrgOwnershipSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/DeleteAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAccountSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeleteAccountRequest>
                    for DeleteAccountSvc<T> {
                        type Response = super::DeleteAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::delete_account(&inner, request).await
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
                        let method = DeleteAccountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.UserService/ChangePassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangePasswordSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ChangePasswordRequest>
                    for ChangePasswordSvc<T> {
                        type Response = super::ChangePasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangePasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::change_password(&inner, request).await
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
                        let method = ChangePasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for UserServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.UserService";
    impl<T> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod workspace_version_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WorkspaceVersionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkspaceVersionServiceClient<tonic::transport::Channel> {
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
    impl<T> WorkspaceVersionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> WorkspaceVersionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            WorkspaceVersionServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVersionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceVersionService/ListVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceVersionService", "ListVersions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_latest(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLatestVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceVersionService/GetLatest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("diag.v1.WorkspaceVersionService", "GetLatest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceVersionService/CreateVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceVersionService", "CreateVersion"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceVersionService/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceVersionService", "GetSettings"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVersionSettingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/diag.v1.WorkspaceVersionService/UpdateSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("diag.v1.WorkspaceVersionService", "UpdateSettings"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod workspace_version_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WorkspaceVersionServiceServer.
    #[async_trait]
    pub trait WorkspaceVersionService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_versions(
            &self,
            request: tonic::Request<super::ListVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVersionsResponse>,
            tonic::Status,
        >;
        async fn get_latest(
            &self,
            request: tonic::Request<super::GetLatestVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLatestVersionResponse>,
            tonic::Status,
        >;
        async fn create_version(
            &self,
            request: tonic::Request<super::CreateVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVersionResponse>,
            tonic::Status,
        >;
        async fn get_settings(
            &self,
            request: tonic::Request<super::GetVersionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionSettingsResponse>,
            tonic::Status,
        >;
        async fn update_settings(
            &self,
            request: tonic::Request<super::UpdateVersionSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVersionSettingsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WorkspaceVersionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> WorkspaceVersionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for WorkspaceVersionServiceServer<T>
    where
        T: WorkspaceVersionService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/diag.v1.WorkspaceVersionService/ListVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListVersionsSvc<T: WorkspaceVersionService>(pub Arc<T>);
                    impl<
                        T: WorkspaceVersionService,
                    > tonic::server::UnaryService<super::ListVersionsRequest>
                    for ListVersionsSvc<T> {
                        type Response = super::ListVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVersionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceVersionService>::list_versions(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListVersionsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceVersionService/GetLatest" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestSvc<T: WorkspaceVersionService>(pub Arc<T>);
                    impl<
                        T: WorkspaceVersionService,
                    > tonic::server::UnaryService<super::GetLatestVersionRequest>
                    for GetLatestSvc<T> {
                        type Response = super::GetLatestVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLatestVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceVersionService>::get_latest(&inner, request)
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
                        let method = GetLatestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceVersionService/CreateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVersionSvc<T: WorkspaceVersionService>(pub Arc<T>);
                    impl<
                        T: WorkspaceVersionService,
                    > tonic::server::UnaryService<super::CreateVersionRequest>
                    for CreateVersionSvc<T> {
                        type Response = super::CreateVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceVersionService>::create_version(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateVersionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceVersionService/GetSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetSettingsSvc<T: WorkspaceVersionService>(pub Arc<T>);
                    impl<
                        T: WorkspaceVersionService,
                    > tonic::server::UnaryService<super::GetVersionSettingsRequest>
                    for GetSettingsSvc<T> {
                        type Response = super::GetVersionSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionSettingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceVersionService>::get_settings(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetSettingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/diag.v1.WorkspaceVersionService/UpdateSettings" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSettingsSvc<T: WorkspaceVersionService>(pub Arc<T>);
                    impl<
                        T: WorkspaceVersionService,
                    > tonic::server::UnaryService<super::UpdateVersionSettingsRequest>
                    for UpdateSettingsSvc<T> {
                        type Response = super::UpdateVersionSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVersionSettingsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as WorkspaceVersionService>::update_settings(
                                        &inner,
                                        request,
                                    )
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
                        let method = UpdateSettingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for WorkspaceVersionServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "diag.v1.WorkspaceVersionService";
    impl<T> tonic::server::NamedService for WorkspaceVersionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
