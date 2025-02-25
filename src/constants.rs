pub const APPLICATION_JSON: &str = "application/json";
pub const TEXT_HTML: &str = "text/html";
pub const USER_AGENT_REGEX: &str = "regexes.yaml";
pub const REMOTE_ADDR: &str = "Remote-Addr";

pub const DEFAULT_CAPACITY: usize = 10;
pub const DEFAULT_FILL_RATE: usize = 2;

pub fn get_ip_url(ip: String) -> String {
    return format!("http://ip-api.com/json/{}", ip);
}