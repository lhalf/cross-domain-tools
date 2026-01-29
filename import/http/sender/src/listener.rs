use crate::config::Config;
use crate::responses::{ReceiveResponse, ResponseMap};
use common::W6300_BUFFER_SIZE;
use common::payload::ExportPayload;

pub async fn run(config: Config, response_map: ResponseMap) -> anyhow::Result<()> {
    let listener = tokio::net::UdpSocket::bind(config.export_address).await?;

    let mut buffer = [0u8; W6300_BUFFER_SIZE];

    loop {
        let (len, _) = listener.recv_from(&mut buffer).await?;
        // TODO: fix this shit
        let data = buffer[..len].to_vec();

        tokio::spawn(on_response_received(data, response_map.clone()));
    }
}

async fn on_response_received<RR: ReceiveResponse>(
    received: Vec<u8>,
    response_map: RR,
) -> anyhow::Result<()> {
    let export_payload: ExportPayload = postcard::from_bytes(&received)?;

    log::debug!("received bytes: {:?}", export_payload.uuid);

    response_map
        .receive_response(export_payload.uuid, export_payload.response)
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::listener::on_response_received;
    use crate::responses::ReceiveResponseSpy;
    use common::payload::ExportPayload;

    #[tokio::test]
    async fn receiving_invalid_request_bytes_returns_error() {
        assert!(
            on_response_received(vec![], ReceiveResponseSpy::default())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn receiving_valid_request_bytes_notifies_response_map() {
        let response_map_spy = ReceiveResponseSpy::default();

        response_map_spy.receive_response.returns.set([()]);

        assert!(
            on_response_received(export_payload_bytes(), response_map_spy)
                .await
                .is_ok()
        );
    }

    fn export_payload_bytes() -> Vec<u8> {
        postcard::to_stdvec(&ExportPayload::default()).unwrap()
    }
}
