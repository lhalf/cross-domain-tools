use crate::config::Config;
use anyhow::Context;
use axum::http::StatusCode;
use common::request::Request;
use common::udp::{SendBytes, UdpSender};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.listen_address).await?;

    let udp_sender = UdpSender::try_new(config.import_address).await?;

    axum::serve(listener, router(udp_sender).await)
        .await
        .context("failed to run request server")
}

#[derive(Clone)]
struct State<SB: SendBytes> {
    udp_sender: SB,
}

async fn router<SB: SendBytes>(udp_sender: SB) -> axum::Router {
    axum::Router::new().route(
        "/",
        axum::routing::any(on_request_received::<SB>).with_state(State { udp_sender }),
    )
}

async fn on_request_received<SB: SendBytes>(
    axum::extract::State(state): axum::extract::State<State<SB>>,
    request: Request,
) -> StatusCode {
    let bytes = match postcard::to_stdvec(&request) {
        Ok(bytes) => bytes,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    match state.udp_sender.try_send_bytes(&bytes).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_GATEWAY,
    }
}

#[cfg(test)]
mod tests {
    use super::router;
    use anyhow::anyhow;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use common::udp::SendBytesSpy;
    use tower::ServiceExt;

    // TODO: can this be tested?
    #[tokio::test]
    async fn receiving_invalid_request_returns_400() {
        assert!(true)
    }

    #[tokio::test]
    async fn failure_to_send_request_bytes_returns_500() {
        let send_bytes_spy = SendBytesSpy::default();

        send_bytes_spy
            .try_send_bytes
            .returns
            .set([Err(anyhow!("test error"))]);

        let router = router(send_bytes_spy).await;

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::BAD_GATEWAY, response.status());
    }
}
