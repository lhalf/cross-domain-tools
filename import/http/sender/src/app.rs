use crate::config::Config;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    super::server::request::run(config).await
}
