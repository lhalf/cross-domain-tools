mod app;
mod config;
mod listener;
mod send_request;
mod send_response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("starting import http receiver...");
    app::run().await
}
