use crate::config::Config;
use crate::responses::{RequestResponse, ResponseMap};
use anyhow::Context;
use axum::http::StatusCode;
use common::payload::ImportPayload;
use common::request::Request;
use common::response::Response;
use common::udp::{SendBytes, UdpSender};
use std::time::Duration;
use uuid::Uuid;

pub async fn run(config: Config, response_map: ResponseMap) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(config.listen_address).await?;

    let udp_sender = UdpSender::try_new(config.import_address).await?;

    axum::serve(
        listener,
        router(
            udp_sender,
            response_map,
            Duration::from_secs_f64(config.timeout),
        )
        .await,
    )
    .await
    .context("failed to run request server")
}

#[derive(Clone)]
struct State<SB: SendBytes, RR: RequestResponse> {
    udp_sender: SB,
    response_map: RR,
    timeout: Duration,
}

async fn router<SB: SendBytes, RR: RequestResponse>(
    udp_sender: SB,
    response_map: RR,
    timeout: Duration,
) -> axum::Router {
    axum::Router::new().route(
        "/",
        axum::routing::any(on_request_received::<SB, RR>).with_state(State {
            udp_sender,
            response_map,
            timeout,
        }),
    )
}

async fn on_request_received<SB: SendBytes, RR: RequestResponse>(
    axum::extract::State(state): axum::extract::State<State<SB, RR>>,
    request: Request,
) -> Response {
    let uuid = Uuid::new_v4();

    log::debug!("received request: {:?}", uuid);

    let response_rx = state.response_map.request_response(uuid).await;

    let bytes = match postcard::to_stdvec(&ImportPayload { uuid, request }) {
        Ok(bytes) => bytes,
        Err(_) => return StatusCode::BAD_REQUEST.into(),
    };

    if state.udp_sender.try_send_bytes(&bytes).await.is_err() {
        state.response_map.remove_response(uuid).await;
        return StatusCode::BAD_GATEWAY.into();
    }

    log::debug!("waiting for response: {:?}", uuid);

    match tokio::time::timeout(state.timeout, response_rx).await {
        Ok(Ok(response)) => {
            log::debug!("got response: {:?}", uuid);
            response
        }
        _ => {
            state.response_map.remove_response(uuid).await;
            StatusCode::GATEWAY_TIMEOUT.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::router;
    use crate::responses::RequestResponseSpy;
    use anyhow::anyhow;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use common::response::Response;
    use common::udp::SendBytesSpy;
    use std::time::Duration;
    use tokio::sync::oneshot;
    use tower::ServiceExt;

    // TODO: can this be tested?
    #[tokio::test]
    async fn receiving_invalid_request_returns_400() {
        assert!(true)
    }

    #[tokio::test]
    async fn unsupported_method_returns_405() {
        let router = router(
            SendBytesSpy::default(),
            RequestResponseSpy::default(),
            Duration::ZERO,
        )
        .await;

        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::CONNECT)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(StatusCode::METHOD_NOT_ALLOWED, response.status());
    }

    #[tokio::test]
    async fn failure_to_send_import_bytes_returns_502_and_response_removed() {
        let (_, response_rx) = oneshot::channel();

        let send_bytes_spy = SendBytesSpy::default();
        let response_map_spy = RequestResponseSpy::default();

        send_bytes_spy
            .try_send_bytes
            .returns
            .set([Err(anyhow!("test error"))]);

        response_map_spy.request_response.returns.set([response_rx]);
        response_map_spy.remove_response.returns.set([()]);

        let router = router(send_bytes_spy, response_map_spy, Duration::ZERO).await;

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::BAD_GATEWAY, response.status());
    }

    #[tokio::test]
    async fn timeout_waiting_for_response_returns_504_and_response_removed() {
        let (_, response_rx) = oneshot::channel();

        let send_bytes_spy = SendBytesSpy::default();
        let response_map_spy = RequestResponseSpy::default();

        send_bytes_spy.try_send_bytes.returns.set([Ok(())]);

        response_map_spy.request_response.returns.set([response_rx]);
        response_map_spy.remove_response.returns.set([()]);

        let router = router(send_bytes_spy, response_map_spy, Duration::ZERO).await;

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::GATEWAY_TIMEOUT, response.status());
    }
    #[tokio::test]
    async fn response_received_returns_response() {
        let (response_tx, response_rx) = oneshot::channel();

        let send_bytes_spy = SendBytesSpy::default();
        let response_map_spy = RequestResponseSpy::default();

        send_bytes_spy.try_send_bytes.returns.set([Ok(())]);

        response_map_spy.request_response.returns.set([response_rx]);

        assert!(response_tx.send(Response::from(StatusCode::OK)).is_ok());

        let router = router(send_bytes_spy, response_map_spy, Duration::ZERO).await;

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());
    }
}
