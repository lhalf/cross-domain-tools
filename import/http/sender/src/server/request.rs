use crate::config::Config;
use crate::request::Request;
use crate::udp::{SendRequest, UdpSender};
use anyhow::Context;
use axum::http::StatusCode;

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.listen_address).await?;

    let udp_sender = UdpSender::try_new(config).await?;

    axum::serve(listener, router(udp_sender).await)
        .await
        .context("failed to run request server")
}

#[derive(Clone)]
struct State<SR: SendRequest> {
    udp_sender: SR,
}

async fn router<SR: SendRequest>(udp_sender: SR) -> axum::Router {
    axum::Router::new().route(
        "/",
        axum::routing::any(on_request_received::<SR>).with_state(State { udp_sender }),
    )
}

async fn on_request_received<SR: SendRequest>(
    axum::extract::State(state): axum::extract::State<State<SR>>,
    request: Request,
) -> StatusCode {
    match state.udp_sender.try_send_request_as_bytes(request).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_GATEWAY,
    }
}

#[cfg(test)]
mod tests {
    use super::router;
    use crate::udp::SendRequestSpy;
    use anyhow::anyhow;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn failure_to_send_request_as_udp_returns_500() {
        let send_request_spy = SendRequestSpy::default();

        send_request_spy
            .try_send_request_as_bytes
            .returns
            .set([Err(anyhow!("test error"))]);

        let router = router(send_request_spy).await;

        let response = router.oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(StatusCode::BAD_GATEWAY, response.status());
    }
}
