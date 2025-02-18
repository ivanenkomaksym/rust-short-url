use actix_web::http::header::HeaderMap;
use user_agent_parser::UserAgentParser;
use reqwest::blocking::get;
use serde_json::Value;

use crate::models::analytic::{self, Analytic};

pub(crate) async fn collect_stats(headers: &HeaderMap) -> Analytic {
    let language = extract_language(&headers);
    let ip = extract_ip(&headers);
    let os = extract_os(&headers);
    let location = ip.as_deref().map_or(None, extract_geolocation);

    let analytic = analytic::Analytic {
        language,
        ip,
        os,
        location
    };

    analytic
}

fn extract_language(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Accept-Language")
        .and_then(|lang| lang.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}

fn extract_os(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get("User-Agent") {
        if let Ok(ua_str) = user_agent.to_str() {
            let ua_parser = UserAgentParser::from_path("regexes.yaml").unwrap();
            return Some(ua_parser.parse_os(ua_str).name.unwrap().to_string());
        }
    }
    None
}

fn extract_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-Forwarded-For") // Check if behind a proxy
        .and_then(|ip| ip.to_str().ok())
        .or_else(|| headers.get("Remote-Addr").and_then(|ip| ip.to_str().ok()))
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}

fn extract_geolocation(ip: &str) -> Option<String> {
    let url = format!("http://ip-api.com/json/{}", ip);
    if let Ok(response) = get(&url) {
        if let Ok(json) = response.json::<Value>() {
            if let (Some(country), Some(city)) = (json["country"].as_str(), json["city"].as_str()) {
                return Some(format!("{}, {}", city, country));
            }
        }
    }
    None
}