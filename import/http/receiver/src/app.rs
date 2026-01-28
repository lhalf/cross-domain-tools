use crate::config::Config;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    super::listener::run(&config).await
}
