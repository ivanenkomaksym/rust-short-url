use std::future::{ready, Ready};
use actix_web::{error, Result, Error, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, HttpResponse, http::{StatusCode, header::ContentType}};
use derive_more::{Display, Error};
use futures_util::future::LocalBoxFuture;

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

struct RateLimiter {
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware { service }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
}

impl<S> RateLimiterMiddleware<S> {
    pub fn new(service: S) -> Self {
        RateLimiterMiddleware { service }
    }
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
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