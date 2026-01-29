mod app;
mod config;
mod listener;
mod responses;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("starting import http sender...");
    app::run().await
}
