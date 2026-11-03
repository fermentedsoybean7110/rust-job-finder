use std::net::SocketAddr;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_API_BASE_URL: &str = "https://vibe-reach.net/api/v1";

pub struct Config {
    pub addr: SocketAddr,
    pub api_base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| DEFAULT_ADDR.to_string())
            .parse()
            .expect("BIND_ADDR must be a valid socket address");
        let api_base_url = std::env::var("VIBE_REACH_API_BASE")
            .unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string());
        Self { addr, api_base_url }
    }
}
