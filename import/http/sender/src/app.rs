use crate::config::Config;
use crate::{listener, server};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    spawn_tasks(config).await
}

async fn spawn_tasks(config: Config) -> anyhow::Result<()> {
    let mut tasks = tokio::task::JoinSet::new();

    tasks.spawn(server::run(config.clone()));
    tasks.spawn(listener::run(config));

    match tasks.join_next().await {
        Some(result) => result?,
        None => unreachable!("always one task"),
    }
}
