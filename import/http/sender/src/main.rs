mod app;
mod config;
mod request;
mod server;
mod udp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("starting import http sender...");
    app::run().await
}
