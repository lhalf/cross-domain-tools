use crate::config::Config;
use common::request::Request;
use common::response::Response;
use std::net::SocketAddrV4;
use std::time::Duration;

#[derive(Clone)]
pub struct HTTPClient {
    client: reqwest::Client,
    target_address: SocketAddrV4,
}

impl HTTPClient {
    pub fn try_new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs_f64(config.timeout))
                .build()?,
            target_address: config.target_address,
        })
    }
}

#[cfg_attr(test, autospy::autospy)]
#[async_trait::async_trait]
pub trait SendRequest: Send + Sync + 'static {
    async fn try_send_request(&self, request: Request) -> anyhow::Result<Response>;
}

#[async_trait::async_trait]
impl SendRequest for HTTPClient {
    async fn try_send_request(&self, request: Request) -> anyhow::Result<Response> {
        self.client
            .execute(request.try_to_reqwest(self.target_address)?)
            .await?
            .try_into()
    }
}
