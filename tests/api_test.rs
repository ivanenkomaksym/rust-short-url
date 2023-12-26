#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, middleware, dev::Service, http};
    use std::sync::{Arc, Mutex};
    use rust_short_url::{services::hashservicefactory::create_hash_service, api::{httpserver::{hello, shorten, redirect, summary, AppData}, ratelimitermiddleware::{RateLimiterMiddlewareService, UserError}, ratelimiter::RateLimiter}, configuration::settings::{Settings, ApiServer, RateLimit}};

    #[actix_web::test]
    async fn test_index_get() {
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await;
        let hash_service_arc = Arc::new(Mutex::new(hash_service));
        
        let app = test::init_service({
            App::new()
                // enable logger - always register actix-web Logger middleware last
                .wrap(middleware::Logger::default())
                // register HTTP requests handlers
                .service(hello)
                .service(web::resource("/shorten").route(web::get().to(shorten)))
                .service(redirect)
                .service(summary)
                .app_data(web::Data::new(hash_service_arc.clone()))
        }).await;
        
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_rate_limit() {
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await;
        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(settings.ratelimit)));
        let appdata = web::Data::new(Mutex::new(AppData { settings, hash_service }));

        let app = test::init_service({
            App::new()
                // enable logger - always register actix-web Logger middleware last
                .wrap(middleware::Logger::default())
                // register HTTP requests handlers
                .service(hello)
                .service(web::resource("/shorten").wrap_fn(move|req, srv| 
                    {
                        let rate_limiter = rate_limiter.clone();
                        RateLimiterMiddlewareService::new(srv, rate_limiter).call(req)
                    }).route(web::get().to(shorten)))
                .service(redirect)
                .service(summary)
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        let req = test::TestRequest::get().uri("/shorten?long_url=https://doc.rust-lang.org/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let req = test::TestRequest::get().uri("/shorten?long_url=https://doc.rust-lang.org/2").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let req = test::TestRequest::get().uri("/shorten?long_url=https://doc.rust-lang.org/3").to_request();
        let resp = test::try_call_service(&app, req).await.err();
        match resp {
            Some(err) => {
                let expected_status = http::StatusCode::TOO_MANY_REQUESTS;
                let actual_status = err.as_response_error().status_code();

                assert_eq!(actual_status, expected_status);

                let err_response = err.to_string();
                let expected_message = UserError::TooManyRequests.to_string();
                assert_eq!(err_response, expected_message);
            }
            None => {
                panic!("Service call succeeded, but an error was expected.");
            }
        }

    }
    
    fn setup_settings() -> Settings {
        return Settings {
            debug: true,
            apiserver: ApiServer { 
                application_url: String::from("localhost"),
                hostname: String::from("localhost")
            },
            database: None,
            ratelimit: Some(RateLimit {capacity: 2, fill_rate: 2}),
            mode: rust_short_url::configuration::settings::Mode::InMemory,
            coordinator: None
        }
    }
}