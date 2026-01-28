use anyhow::Context;
use serde::Deserialize;
use std::net::SocketAddrV4;
use std::time::Duration;

const PATH: &str = "/etc/import-http/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub listen_address: SocketAddrV4,
    pub export_address: SocketAddrV4,
    pub timeout: Duration,
}

impl Config {
    pub fn try_load() -> anyhow::Result<Self> {
        toml::from_str(&std::fs::read_to_string(PATH)?).context("failed to load config")
    }
}
