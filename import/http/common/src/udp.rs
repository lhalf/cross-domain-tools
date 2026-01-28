use std::net::SocketAddrV4;
use std::sync::Arc;
use tokio::net::UdpSocket;

const ADDRESS: &str = "0.0.0.0:0";

#[derive(Clone)]
pub struct UdpSender {
    pub socket: Arc<UdpSocket>,
    pub address: SocketAddrV4,
}

impl UdpSender {
    pub async fn try_new(address: SocketAddrV4) -> anyhow::Result<Self> {
        Ok(Self {
            socket: Arc::new(UdpSocket::bind(ADDRESS).await?),
            address,
        })
    }
}
