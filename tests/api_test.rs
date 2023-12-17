#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, middleware};
    use std::sync::{Arc, Mutex};
    use rust_short_url::{services::hashservicefactory::create_hash_service, api::httpserver::{hello, shorten, redirect, summary}, configuration::settings::{Settings, ApiServer}};

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
    
    fn setup_settings() -> Settings {
        return Settings { debug: true, apiserver: ApiServer { application_url: String::from("localhost"), hostname: String::from("localhost") }, database: None, ratelimit: None }
    }
}