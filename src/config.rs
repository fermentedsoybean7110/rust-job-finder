use std::net::SocketAddr;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";

pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        let addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| DEFAULT_ADDR.to_string())
            .parse()
            .expect("BIND_ADDR must be a valid socket address");
        Self { addr }
    }
}
