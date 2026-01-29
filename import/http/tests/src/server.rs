use axum::Router;
use axum::routing::get;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

const ADDRESS: &str = "0.0.0.0:9002";

pub struct Server {
    stop: Option<oneshot::Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

impl Server {
    pub fn spawn() -> Self {
        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        Self {
            stop: Some(stop_tx),
            handle: Some(std::thread::spawn(move || Self::serve(stop_rx))),
        }
    }

    fn serve(stop_rx: oneshot::Receiver<()>) {
        Runtime::new().unwrap().block_on(async {
            let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

            axum::serve(listener, Self::router())
                .with_graceful_shutdown(async {
                    stop_rx.await.ok();
                })
                .await
                .unwrap()
        })
    }

    fn router() -> Router {
        Router::new().route("/", get(|| async {}))
    }

    fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            stop.send(()).unwrap();
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}
