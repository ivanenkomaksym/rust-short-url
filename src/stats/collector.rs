use actix_web::http::header::{self, HeaderMap};
use user_agent_parser::UserAgentParser;
use serde_json::Value;

use crate::{constants, models::analytic::{self, Analytic}};

pub async fn collect_stats(headers: &HeaderMap) -> Analytic {    
    // Debug: print all headers for complete analysis
    log::debug!("=== All Headers ===");
    for (name, value) in headers.iter() {
        log::debug!("{}: {:?}", name, value);
    }
    let language = extract_language(&headers);
    let ip = extract_ip(&headers);
    let os = extract_os(&headers);
    let referrer = extract_referrer(&headers);
    let device_type = extract_device_type(&headers);
    let browser = extract_browser(&headers);
    let user_agent = extract_user_agent(&headers);
    let location = match ip.clone() {
        Some(value) => extract_geolocation(&value).await,
        None => None        
    };

    let analytic = analytic::Analytic {
        created_at: chrono::Utc::now(),
        language,
        ip,
        os,
        location,
        referrer,
        device_type,
        browser,
        user_agent,
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
    // Debug: print IP-related headers specifically
    log::info!("=== IP Detection Headers ===");
    for header_name in &["CF-Connecting-IP", "True-Client-IP", "X-Real-IP", "X-Forwarded-For", "Remote-Addr"] {
        if let Some(value) = headers.get(*header_name) {
            log::info!("{}: {:?}", header_name, value);
        }
    }
    // Priority order for IP extraction (most trusted first)
    
    // 1. Cloudflare's connecting IP (most reliable when behind Cloudflare)
    if let Some(ip) = headers.get("CF-Connecting-IP")
        .and_then(|ip| ip.to_str().ok())
        .map(|s| s.trim().to_string()) {
        if !ip.is_empty() && is_valid_ip(&ip) {
            return Some(ip);
        }
    }
    
    // 2. True-Client-IP (used by some CDNs)
    if let Some(ip) = headers.get("True-Client-IP")
        .and_then(|ip| ip.to_str().ok())
        .map(|s| s.trim().to_string()) {
        if !ip.is_empty() && is_valid_ip(&ip) {
            return Some(ip);
        }
    }
    
    // 3. X-Real-IP (used by nginx and other proxies)
    if let Some(ip) = headers.get("X-Real-IP")
        .and_then(|ip| ip.to_str().ok())
        .map(|s| s.trim().to_string()) {
        if !ip.is_empty() && is_valid_ip(&ip) {
            return Some(ip);
        }
    }
    
    // 4. X-Forwarded-For (can contain multiple IPs, take the first/leftmost)
    if let Some(ip) = headers.get(header::X_FORWARDED_FOR.as_str())
        .and_then(|ip| ip.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string()) {
        if !ip.is_empty() && is_valid_ip(&ip) {
            return Some(ip);
        }
    }
    
    // 5. Fallback to remote address (direct connection)
    headers.get(constants::REMOTE_ADDR)
        .and_then(|ip| ip.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|ip| !ip.is_empty() && is_valid_ip(ip))
}

fn is_valid_ip(ip: &str) -> bool {
    // Basic IP validation - check if it's a valid IPv4 or IPv6
    if ip.contains(':') {
        // IPv6 basic validation
        ip.chars().all(|c| c.is_ascii_hexdigit() || c == ':') && ip.matches(':').count() >= 2
    } else {
        // IPv4 validation
        ip.split('.')
            .filter_map(|part| part.parse::<u8>().ok())
            .count() == 4
    }
}

fn extract_referrer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::REFERER.as_str())
        .and_then(|referrer| referrer.to_str().ok())
        .map(|s| s.to_string())
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::USER_AGENT.as_str())
        .and_then(|ua| ua.to_str().ok())
        .map(|s| s.to_string())
}

fn extract_browser(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get(header::USER_AGENT.as_str()) {
        if let Ok(ua_str) = user_agent.to_str() {
            let ua_lower = ua_str.to_lowercase();
            
            // Check for different browsers in order of specificity
            if ua_lower.contains("edg/") || ua_lower.contains("edge/") {
                return Some("Microsoft Edge".to_string());
            } else if ua_lower.contains("chrome/") && !ua_lower.contains("chromium/") {
                return Some("Chrome".to_string());
            } else if ua_lower.contains("firefox/") {
                return Some("Firefox".to_string());
            } else if ua_lower.contains("safari/") && !ua_lower.contains("chrome") {
                return Some("Safari".to_string());
            } else if ua_lower.contains("opera/") || ua_lower.contains("opr/") {
                return Some("Opera".to_string());
            } else if ua_lower.contains("chromium/") {
                return Some("Chromium".to_string());
            } else if ua_lower.contains("trident/") || ua_lower.contains("msie") {
                return Some("Internet Explorer".to_string());
            }
            
            return Some("Unknown".to_string());
        }
    }
    None
}

fn extract_device_type(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get(header::USER_AGENT.as_str()) {
        if let Ok(ua_str) = user_agent.to_str() {
            let ua_lower = ua_str.to_lowercase();
            
            // Check for mobile devices
            if ua_lower.contains("mobile") || ua_lower.contains("android") || 
               ua_lower.contains("iphone") || ua_lower.contains("ipod") ||
               ua_lower.contains("blackberry") || ua_lower.contains("windows phone") {
                return Some("Mobile".to_string());
            }
            
            // Check for tablets
            if ua_lower.contains("tablet") || ua_lower.contains("ipad") ||
               ua_lower.contains("kindle") || ua_lower.contains("silk") {
                return Some("Tablet".to_string());
            }
            
            // Check for smart TVs
            if ua_lower.contains("smart-tv") || ua_lower.contains("googletv") ||
               ua_lower.contains("appletv") || ua_lower.contains("hbbtv") {
                return Some("Smart TV".to_string());
            }
            
            // Check for gaming consoles
            if ua_lower.contains("playstation") || ua_lower.contains("xbox") ||
               ua_lower.contains("nintendo") {
                return Some("Gaming Console".to_string());
            }
            
            // Check for bots/crawlers
            if ua_lower.contains("bot") || ua_lower.contains("crawler") ||
               ua_lower.contains("spider") || ua_lower.contains("scraper") {
                return Some("Bot".to_string());
            }
            
            // Default to desktop
            return Some("Desktop".to_string());
        }
    }
    None
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