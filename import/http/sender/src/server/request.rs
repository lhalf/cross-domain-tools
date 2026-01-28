use crate::config::Config;
use anyhow::Context;

pub async fn run(config: Config) -> anyhow::Result<()> {
    let listener =
        tokio::net::TcpListener::bind((config.listen_address, config.listen_port)).await?;

    axum::serve(listener, router().await)
        .await
        .context("failed to run request server")
}

async fn router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::any(on_request_received))
}

async fn on_request_received() {
    todo!()
}
