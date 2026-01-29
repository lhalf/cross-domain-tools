use crate::config::Config;
use common::request::Request;
use common::response::Response;
use std::time::Duration;

#[derive(Clone)]
pub struct HTTPClient {
    client: reqwest::Client,
}

impl HTTPClient {
    pub fn try_new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs(config.timeout))
                .build()?,
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
        self.client.execute(request.try_into()?).await?.try_into()
    }
}
