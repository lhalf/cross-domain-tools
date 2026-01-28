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

#[cfg_attr(feature = "test", autospy::autospy)]
#[async_trait::async_trait]
pub trait SendBytes: Clone + Send + Sync + 'static {
    async fn try_send_bytes(&self, bytes: &[u8]) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SendBytes for UdpSender {
    async fn try_send_bytes(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let _ = self.socket.send_to(bytes, self.address).await?;
        Ok(())
    }
}
