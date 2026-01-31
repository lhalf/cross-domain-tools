use crate::config::Config;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    #[allow(clippy::large_futures)]
    super::listener::run(&config).await
}
