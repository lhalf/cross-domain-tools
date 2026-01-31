use axum::Router;
use axum::extract::Request;
use axum::extract::State;
use axum::routing::any;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::{Arc, MutexGuard};
use std::thread::JoinHandle;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

const ADDRESS: &str = "127.0.0.1:9002";

pub struct Server {
    stop_tx: Option<oneshot::Sender<()>>,
    handle: Option<JoinHandle<()>>,
    received_requests: Arc<Mutex<Vec<Request>>>,
}

impl Server {
    pub fn spawn() -> Self {
        let (stop_tx, stop_rx) = oneshot::channel::<()>();

        let received_requests = Arc::new(Mutex::new(Vec::new()));
        let received_requests_clone = received_requests.clone();

        Self {
            stop_tx: Some(stop_tx),
            handle: Some(std::thread::spawn(move || {
                Self::serve(stop_rx, received_requests_clone)
            })),
            received_requests,
        }
    }

    fn serve(stop_rx: oneshot::Receiver<()>, received_requests: Arc<Mutex<Vec<Request>>>) {
        Runtime::new().unwrap().block_on(async {
            let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

            axum::serve(listener, Self::router(received_requests))
                .with_graceful_shutdown(async {
                    stop_rx.await.ok();
                })
                .await
                .unwrap()
        })
    }

    fn router(received_requests: Arc<Mutex<Vec<Request>>>) -> Router {
        Router::new().route(
            "/",
            any(Self::default_endpoint).with_state(received_requests.clone()),
        )
    }

    async fn default_endpoint(
        State(received_requests): State<Arc<Mutex<Vec<Request>>>>,
        request: Request,
    ) {
        received_requests.lock().unwrap().push(request)
    }

    pub fn received_requests(&self) -> MutexGuard<'_, Vec<Request>> {
        self.received_requests.lock().unwrap()
    }

    pub fn stop(&mut self) {
        if let Some(stop_tx) = self.stop_tx.take() {
            stop_tx.send(()).unwrap();
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        Self::wait_for_stop()
    }

    fn wait_for_stop() {
        retry::retry(retry::delay::Fixed::from_millis(100).take(10), || {
            TcpStream::connect(ADDRESS).err().map(|_| ()).ok_or(())
        })
        .unwrap();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}
