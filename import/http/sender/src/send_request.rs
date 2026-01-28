use common::request::Request;
use common::udp::UdpSender;

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait SendRequest: Clone + Send + Sync + 'static {
    async fn try_send_request(&self, request: Request) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SendRequest for UdpSender {
    async fn try_send_request(&self, request: Request) -> anyhow::Result<()> {
        let _ = self
            .socket
            .send_to(&postcard::to_stdvec::<Request>(&request)?, self.address)
            .await?;
        Ok(())
    }
}
