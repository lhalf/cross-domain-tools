mod app;
mod config;
mod listener;
mod send_request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("starting import http receiver...");
    #[allow(clippy::large_futures)]
    app::run().await
}
