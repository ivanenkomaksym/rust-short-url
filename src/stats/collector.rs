use actix_web::http::header::HeaderMap;
use user_agent_parser::UserAgentParser;
use std::collections::HashMap;

async fn collect_stats(headers: HeaderMap) {
    let ip = extract_ip(&headers);
    let language = extract_language(&headers);
    let device_info = extract_device_info(&headers);

    let mut stats = HashMap::new();
    stats.insert("IP", ip.unwrap_or_else(|| "Unknown".to_string()));
    stats.insert("Language", language.unwrap_or_else(|| "Unknown".to_string()));
    stats.insert("Device Info", device_info.unwrap_or_else(|| "Unknown".to_string()));

    println!("{:?}", stats); // Store it in DB instead
}

fn extract_language(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Accept-Language")
        .and_then(|lang| lang.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").to_string())
}

fn extract_device_info(headers: &HeaderMap) -> Option<String> {
    if let Some(user_agent) = headers.get("User-Agent") {
        if let Ok(ua_str) = user_agent.to_str() {
            let ua_parser: UserAgentParser = UserAgentParser::from_str(ua_str)?;
            return Some(format!(
                "OS: {:#?}, Device: {:#?}",
                ua_parser.parse_os(ua_str),
                ua_parser.parse_device(ua_str)
            ));
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