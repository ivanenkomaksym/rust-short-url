use std::future::{ready, Ready};
use actix_web::{error, Result, Error, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, HttpResponse, http::{StatusCode, header::ContentType}};
use derive_more::{Display, Error};
use futures_util::future::LocalBoxFuture;
use crate::configuration::settings::RateLimit;

use super::ratelimiter::RateLimiter;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "You have sent too many requests in a given amount of time. Please try again later.")]
    TooManyRequests,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

struct RateLimiterMiddleware {
    rate_limiter: RateLimiter,
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RateLimiterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddlewareService { service, _rate_limiter: self.rate_limiter.clone() }))
    }
}

pub struct RateLimiterMiddlewareService<S> {
    pub service: S,
    pub _rate_limiter: RateLimiter,
}

impl<S> RateLimiterMiddlewareService<S> {
    pub fn new(service: S, rate_limiter_options: RateLimit) -> Self {
        RateLimiterMiddlewareService { service, _rate_limiter: RateLimiter::new(Some(rate_limiter_options)) }
    }
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            // Your rate limiting logic goes here
            // Access the rate limiter instance

            // Check if the request can be processed based on the rate limiter
            if false {
                // Continue processing the request
                let res = fut.await?;
                Ok(res)
            } else {
                Err(Error::from(UserError::TooManyRequests))
            }
        })
    }
}