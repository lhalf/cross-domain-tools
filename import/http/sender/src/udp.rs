use crate::config::Config;
use crate::request::Request;
use anyhow::Context;
use std::net::SocketAddrV4;
use std::sync::Arc;
use tokio::net::UdpSocket;

const ADDRESS: &str = "0.0.0.0:0";

#[derive(Clone)]
pub struct UdpSender {
    socket: Arc<UdpSocket>,
    address: SocketAddrV4,
}

impl UdpSender {
    pub async fn try_new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            socket: Arc::new(UdpSocket::bind(ADDRESS).await?),
            address: config.import_address,
        })
    }
}

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait SendRequest: Clone + Send + Sync + 'static {
    async fn try_send_request_as_bytes(&self, request: Request) -> anyhow::Result<usize>;
}

#[async_trait::async_trait]
impl SendRequest for UdpSender {
    async fn try_send_request_as_bytes(&self, request: Request) -> anyhow::Result<usize> {
        self.socket
            .send_to(&postcard::to_stdvec::<Request>(&request)?, self.address)
            .await
            .context("failed to send request")
    }
}
