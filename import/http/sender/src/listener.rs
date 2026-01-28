use crate::config::Config;
use common::W6300_BUFFER_SIZE;
use common::response::Response;

pub async fn run(config: Config) -> anyhow::Result<()> {
    let listener = tokio::net::UdpSocket::bind(config.export_address).await?;

    let mut buffer = [0u8; W6300_BUFFER_SIZE];

    loop {
        let (len, _) = listener.recv_from(&mut buffer).await?;
        // TODO: fix this shit
        let data = buffer[..len].to_vec();
        tokio::spawn(on_response_received(data));
    }
}

async fn on_response_received(received: Vec<u8>) -> anyhow::Result<()> {
    let _response: Response = postcard::from_bytes(&received)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::listener::on_response_received;

    #[tokio::test]
    async fn receiving_invalid_request_bytes_returns_error() {
        assert!(on_response_received(vec![]).await.is_err());
    }
}
