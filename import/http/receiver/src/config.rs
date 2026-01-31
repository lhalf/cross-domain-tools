use anyhow::Context;
use serde::Deserialize;
use std::net::SocketAddrV4;

const DEFAULT_PATH: &str = "/etc/import-http/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub import_address: SocketAddrV4,
    pub export_address: SocketAddrV4,
    pub timeout: f64,
}

impl Config {
    pub fn try_load() -> anyhow::Result<Self> {
        let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| DEFAULT_PATH.to_string());
        toml::from_str(&std::fs::read_to_string(path)?).context("failed to load config")
    }
}
