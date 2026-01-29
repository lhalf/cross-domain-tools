use crate::config::Config;
use crate::responses::ResponseMap;
use crate::{listener, server};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    spawn_tasks(config).await
}

async fn spawn_tasks(config: Config) -> anyhow::Result<()> {
    let mut tasks = tokio::task::JoinSet::new();

    let response_map = ResponseMap::default();

    tasks.spawn(server::run(config.clone(), response_map.clone()));
    tasks.spawn(listener::run(config, response_map));

    match tasks.join_next().await {
        Some(result) => result?,
        None => unreachable!("always one task"),
    }
}
