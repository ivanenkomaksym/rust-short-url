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
                .service(web::resource("/shorten").route(web::post().to(shorten)))
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
                .service(web::resource("/shorten").route(web::post().to(shorten)))
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        // Act
        let payload = format!(r#"{{"long_url":"{}"}}"#, long_url);
        let req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload)
            .insert_header(("content-type", "application/json"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Assert
        let link_info: LinkInfo = test::read_body_json(resp).await;
        assert_eq!(link_info.long_url, long_url);
    }

    #[actix_web::test]
    async fn test_redirect() {
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
                .service(web::resource("/shorten").route(web::post().to(shorten)))
                .service(redirect)
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        // Act - shorten
        let payload = format!(r#"{{"long_url":"{}"}}"#, long_url);
        let shorten_req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload)
            .insert_header(("content-type", "application/json"))
            .to_request();
        let shorten_req_resp = test::call_service(&app, shorten_req).await;
        assert!(shorten_req_resp.status().is_success());
        let shorten_link_info: LinkInfo = test::read_body_json(shorten_req_resp).await;
        assert_eq!(shorten_link_info.long_url, long_url);
        let short_url = shorten_link_info.short_url;

        // Assert
        let req = test::TestRequest::get().uri(&format!("/{}", short_url)).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }

    #[actix_web::test]
    async fn test_summary() {
        // Arrange
        let long_url = "https://doc.rust-lang.org/1";
        let ip = "192.1.1.1";
        let expected_location = "Cambridge, United States";
        let settings = setup_settings();
        let hash_service = create_hash_service(&settings).await.unwrap();
        let appdata = web::Data::new(Mutex::new(AppData { settings, hash_service }));

        let app = test::init_service({
            App::new()
                // enable logger - always register actix-web Logger middleware last
                .wrap(middleware::Logger::default())
                // register HTTP requests handlers
                .service(web::resource("/shorten").route(web::post().to(shorten)))
                .service(redirect)
                .service(summary)
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        // Act - shorten
        let payload = format!(r#"{{"long_url":"{}"}}"#, long_url);
        let shorten_req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload)
            .insert_header(("content-type", "application/json"))
            .to_request();
        let shorten_req_resp = test::call_service(&app, shorten_req).await;
        assert!(shorten_req_resp.status().is_success());
        let shorten_link_info: LinkInfo = test::read_body_json(shorten_req_resp).await;
        assert_eq!(shorten_link_info.long_url, long_url);
        let short_url = shorten_link_info.short_url;

        // Act - navigate
        let req = test::TestRequest::get()
            .uri(&format!("/{}", short_url))
            .insert_header(("X-Forwarded-For", ip))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());

        // Assert
        let summary_req = test::TestRequest::get().uri(&format!("/{}/summary", short_url)).to_request();
        let summary_resp = test::call_service(&app, summary_req).await;
        assert!(summary_resp.status().is_success());
        let summary_link_info: LinkInfo = test::read_body_json(summary_resp).await;
        assert_eq!(summary_link_info.long_url, long_url);
        let analytics = summary_link_info.analytics.expect("analytics is empty");
        assert_eq!(analytics.is_empty(), false);
        let analytic = &analytics[0];
        assert_eq!(analytic.ip, Some(ip.to_string()));
        assert_eq!(analytic.location, Some(expected_location.to_string()));
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
                    }).route(web::post().to(shorten)))
                .service(redirect)
                .service(summary)
                .app_data(web::Data::clone(&appdata))
        }).await;
        
        let payload1 = r#"{"long_url":"https://doc.rust-lang.org/1"}"#;
        let req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload1)
            .insert_header(("content-type", "application/json"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let payload2 = r#"{"long_url":"https://doc.rust-lang.org/2"}"#;
        let req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload2)
            .insert_header(("content-type", "application/json"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let payload3 = r#"{"long_url":"https://doc.rust-lang.org/3"}"#;
        let req = test::TestRequest::post()
            .uri("/shorten")
            .set_payload(payload3)
            .insert_header(("content-type", "application/json"))
            .to_request();
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
                allow_origin: String::from("localhost"),
                api_key: Some(String::from("testkey")),
                google_application_credentials: Some(String::from("credentials.json")),
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