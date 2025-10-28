use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
};

pub struct ApiKeyMiddleware {
    api_key: Option<String>,
}

impl ApiKeyMiddleware {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddlewareService {
            service,
            api_key: self.api_key.clone(),
        }))
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: S,
    api_key: Option<String>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = self.api_key.clone();
        
        // If no API key is configured, skip authentication
        if api_key.is_none() {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }

        let configured_key = api_key.unwrap();
        
        // Check API key
        let valid_key = if let Some(request_api_key) = req.headers().get("X-API-Key") {
            if let Ok(key_str) = request_api_key.to_str() {
                key_str == configured_key
            } else {
                false
            }
        } else {
            false
        };

        if !valid_key {
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Invalid or missing API key"
                }));
            return Box::pin(async move {
                Ok(req.into_response(response).map_into_right_body())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}