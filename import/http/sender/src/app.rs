use crate::config::Config;
use crate::responses::ResponseMap;
use crate::{listener, server};
use tokio_util::sync::CancellationToken;
use tokio_util::task::task_tracker::TaskTracker;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    let shutdown_token = CancellationToken::new();
    let tasks = spawn_tasks(config, &shutdown_token).await;
    wait_for_shutdown(tasks, &shutdown_token).await
}

async fn spawn_tasks(config: Config, shutdown_token: &CancellationToken) -> TaskTracker {
    let tasks = TaskTracker::new();

    let response_map = ResponseMap::default();

    tasks.spawn(server::run(
        config.clone(),
        response_map.clone(),
        shutdown_token.clone(),
    ));
    tasks.spawn(listener::run(config, response_map, shutdown_token.clone()));

    tasks
}

async fn wait_for_shutdown(
    tasks: TaskTracker,
    shutdown_token: &CancellationToken,
) -> anyhow::Result<()> {
    tokio::signal::ctrl_c().await?;
    log::info!("received shutdown signal, shutting down...");
    shutdown_token.cancel();
    tasks.close();
    tasks.wait().await;
    Ok(())
}
