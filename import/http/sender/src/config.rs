use anyhow::Context;
use serde::Deserialize;
use std::net::Ipv4Addr;

const PATH: &str = "/etc/import-http/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub listen_address: Ipv4Addr,
    pub listen_port: u16,
}

impl Config {
    pub fn try_load() -> anyhow::Result<Self> {
        toml::from_str(&std::fs::read_to_string(PATH)?).context("failed to load config")
    }
}
