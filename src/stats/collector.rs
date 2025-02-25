use actix_web::http::header::{self, HeaderMap};
use user_agent_parser::UserAgentParser;
use serde_json::Value;

use crate::{constants, models::analytic::{self, Analytic}};

pub async fn collect_stats(headers: &HeaderMap) -> Analytic {
    let language = extract_language(&headers);
    let ip = extract_ip(&headers);
    let os = extract_os(&headers);
    let location = match ip.clone() {
        Some(value) => extract_geolocation(&value).await,
        None => None        
    };

    let analytic = analytic::Analytic {
        created_at: chrono::Utc::now(),
        language,
        ip,
        os,
        location
    };

    analytic
}

fn extract_language(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::ACCEPT_LANGUAGE.as_str())
        .and_then(|lang| lang.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}

fn extract_os(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get(header::USER_AGENT.as_str()) {
        if let Ok(ua_str) = user_agent.to_str() {
            let ua_parser = match UserAgentParser::from_path(constants::USER_AGENT_REGEX) {
                Ok(parser) => parser,
                Err(_) => return None
            };
            return Some(ua_parser.parse_os(ua_str).name.unwrap().to_string());
        }
    }
    None
}

fn extract_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::X_FORWARDED_FOR.as_str()) // Check if behind a proxy
        .and_then(|ip| ip.to_str().ok())
        .or_else(|| headers.get(constants::REMOTE_ADDR).and_then(|ip| ip.to_str().ok()))
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}

async fn extract_geolocation(ip: &str) -> Option<String> {
    let url = constants::get_ip_url(ip.to_string());
    if let Ok(response) = reqwest::get(&url).await {
        if let Ok(json) = response.json::<Value>().await {
            if let (Some(country), Some(city)) = (json["country"].as_str(), json["city"].as_str()) {
                return Some(format!("{}, {}", city, country));
            }
        }
    }
    None
}