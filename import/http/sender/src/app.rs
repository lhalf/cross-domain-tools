use crate::config::Config;
use crate::responses::ResponseMap;
use crate::{listener, server};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::try_load()?;
    let shutdown_token = CancellationToken::new();
    let mut tasks = spawn_tasks(config, &shutdown_token).await;
    wait_for_shutdown(&mut tasks, &shutdown_token).await
}

async fn spawn_tasks(
    config: Config,
    shutdown_token: &CancellationToken,
) -> JoinSet<anyhow::Result<()>> {
    let mut tasks = JoinSet::new();

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
    tasks: &mut JoinSet<anyhow::Result<()>>,
    shutdown_token: &CancellationToken,
) -> anyhow::Result<()> {
    tokio::signal::ctrl_c().await?;

    log::info!("received shutdown signal, shutting down...");

    shutdown_token.cancel();

    while let Some(result) = tasks.join_next().await {
        let _ = result?;
    }

    Ok(())
}
