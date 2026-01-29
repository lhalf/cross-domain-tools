use common::response::Response;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::oneshot::{Receiver, Sender};
use uuid::Uuid;

pub type ResponseMap = Arc<Mutex<HashMap<Uuid, Sender<Response>>>>;

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait RequestResponse: Clone + Send + Sync + 'static {
    async fn request_response(&self, uuid: Uuid) -> Receiver<Response>;
    async fn remove_response(&self, uuid: Uuid);
}

#[async_trait::async_trait]
impl RequestResponse for ResponseMap {
    async fn request_response(&self, uuid: Uuid) -> Receiver<Response> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let _ = self.lock().await.insert(uuid, sender);
        receiver
    }

    async fn remove_response(&self, uuid: Uuid) {
        let _ = self.lock().await.remove(&uuid);
    }
}

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait ReceiveResponse: Clone + Send + Sync + 'static {
    async fn receive_response(&self, uuid: Uuid, response: Response);
}

#[async_trait::async_trait]
impl ReceiveResponse for ResponseMap {
    async fn receive_response(&self, uuid: Uuid, response: Response) {
        if let Some(sender) = self.lock().await.remove(&uuid) {
            let _ = sender.send(response);
        }
    }
}
