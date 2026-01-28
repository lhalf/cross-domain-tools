#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("starting http import sender...");
    Ok(())
}
