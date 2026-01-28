use common::response::Response;
use common::udp::UdpSender;

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait SendResponse {
    async fn try_send_response(&self, response: Response) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SendResponse for UdpSender {
    async fn try_send_response(&self, response: Response) -> anyhow::Result<()> {
        let _ = self
            .socket
            .send_to(&postcard::to_stdvec::<Response>(&response)?, self.address)
            .await?;
        Ok(())
    }
}
