#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, middleware, dev::Service, http};
    use std::sync::{Arc, Mutex};
    use rust_short_url::{api::{httpserver::{hello, redirect, shorten, summary, AppData}, ratelimiter::RateLimiter, ratelimitermiddleware::{RateLimiterMiddlewareService, UserError}}, configuration::settings::{ApiServer, RateLimit, Settings}, models::linkinfo::LinkInfo, services::hashservicefactory::create_hash_service};

    #[actix_web::test]
    async fn test_index_get() {
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await.unwrap();
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
    async fn test_shorten() {
        // Arrange
        let long_url = "https://doc.rust-lang.org/1";
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await.unwrap();
        let appdata = web::Data::new(Mutex::new(AppData { settings, hash_service }));

        let app = test::init_service({
            App::new()
                // enable logger - always register actix-web Logger middleware last
                .wrap(middleware::Logger::default())
                // register HTTP requests handlers
                .service(web::resource("/shorten").route(web::get().to(shorten)))
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        // Act
        let req = test::TestRequest::get().uri(&format!("/shorten?long_url={}", long_url)).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Assert
        let link_info: LinkInfo = test::read_body_json(resp).await;
        assert_eq!(link_info.long_url, long_url);
    }

    #[actix_web::test]
    async fn test_rate_limit() {
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await.unwrap();
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
                hostname: String::from("localhost"),
                allow_origin: String::from("localhost")
            },
            mongo_config: None,
            firestore_config: None,
            redis_config: None,
            ratelimit: Some(RateLimit {capacity: 2, fill_rate: 2}),
            mode: rust_short_url::configuration::settings::Mode::InMemory,
            coordinator: None
        }
    }
}