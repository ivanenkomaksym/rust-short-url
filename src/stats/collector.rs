use actix_web::http::header::HeaderMap;
use std::collections::HashMap;

async fn collect_stats(headers: HeaderMap) {
    let language = extract_language(&headers);

    let mut stats = HashMap::new();
    stats.insert("Language", language.unwrap_or_else(|| "Unknown".to_string()));

    println!("{:?}", stats); // Store it in DB instead
}

fn extract_language(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Accept-Language")
        .and_then(|lang| lang.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}