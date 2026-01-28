use crate::config::Config;
use crate::send_request::{HTTPClient, SendRequest};
use common::W6300_BUFFER_SIZE;
use common::request::Request;
use common::udp::SendBytes;
use common::udp::UdpSender;

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let listener = tokio::net::UdpSocket::bind(config.listen_address).await?;
    let http_client = HTTPClient::try_new(config)?;
    let udp_sender = UdpSender::try_new(config.export_address).await?;
    let mut buffer = [0u8; W6300_BUFFER_SIZE];

    loop {
        let (len, _) = listener.recv_from(&mut buffer).await?;
        // TODO: fix this shit
        let data = buffer[..len].to_vec();
        tokio::spawn(on_request_received(
            data,
            http_client.clone(),
            udp_sender.clone(),
        ));
    }
}

async fn on_request_received<SRQ: SendRequest, SB: SendBytes>(
    received: Vec<u8>,
    http_client: SRQ,
    udp_sender: SB,
) -> anyhow::Result<()> {
    let response = http_client
        .try_send_request(postcard::from_bytes::<Request>(&received)?)
        .await?;

    let bytes = postcard::to_stdvec(&response)?;

    udp_sender.try_send_bytes(&bytes).await
}

#[cfg(test)]
mod tests {
    use super::on_request_received;
    use crate::send_request::SendRequestSpy;
    use anyhow::anyhow;
    use common::response::Response;
    use common::udp::SendBytesSpy;

    #[tokio::test]
    async fn receiving_invalid_request_bytes_returns_error() {
        assert!(
            on_request_received(
                b"i am invalid".to_vec(),
                SendRequestSpy::default(),
                SendBytesSpy::default()
            )
            .await
            .is_err()
        );
    }

    #[tokio::test]
    async fn failure_to_send_request_returns_error() {
        let send_request_spy = SendRequestSpy::default();

        send_request_spy
            .try_send_request
            .returns
            .set([Err(anyhow!("test error"))]);

        assert!(
            on_request_received(vec![0], send_request_spy, SendBytesSpy::default())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn failure_to_send_response_bytes_returns_error() {
        let send_request_spy = SendRequestSpy::default();
        let send_bytes_spy = SendBytesSpy::default();

        send_request_spy
            .try_send_request
            .returns
            .set([Ok(Response::default())]);

        send_bytes_spy
            .try_send_bytes
            .returns
            .set([Err(anyhow!("test error"))]);

        assert!(
            on_request_received(vec![0], send_request_spy, send_bytes_spy)
                .await
                .is_err()
        );
    }
}
