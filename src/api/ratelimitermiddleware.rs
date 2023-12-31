use std::sync::{Mutex, Arc};
use actix_web::{error, Result, Error, dev::{forward_ready, Service, ServiceRequest, ServiceResponse}, HttpResponse, http::{StatusCode, header::ContentType}};
use derive_more::{Display, Error};
use futures_util::future::LocalBoxFuture;

use super::ratelimiter::RateLimiter;

#[derive(Debug, Display, Error)]
pub enum UserError {
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

pub struct RateLimiterMiddlewareService<S> {
    pub service: S,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl<S> RateLimiterMiddlewareService<S> {
    pub fn new(service: S, rate_limiter: Arc<Mutex<RateLimiter>>) -> Self {
        RateLimiterMiddlewareService { service, rate_limiter }
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
        let rate_limiter = self.rate_limiter.clone();

        Box::pin(async move {
            // Access the rate limiter instance
            let mut ratelimiter = rate_limiter.lock().unwrap();

            // Check if the request can be processed based on the rate limiter
            if ratelimiter.consume(1) {
                // Continue processing the request
                let res = fut.await?;
                Ok(res)
            } else {
                Err(Error::from(UserError::TooManyRequests))
            }
        })
    }
}