use axum::http::StatusCode;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Request {
    pub method: Method,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Method {
    #[default]
    Get,
}

impl<S: Sync> axum::extract::FromRequest<S> for Request {
    type Rejection = StatusCode;

    async fn from_request(
        _request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Request::default())
    }
}

impl TryFrom<Request> for reqwest::Request {
    type Error = anyhow::Error;

    fn try_from(_value: Request) -> anyhow::Result<Self> {
        Ok(Self::new(
            reqwest::Method::GET,
            Url::parse("https://localhost")?,
        ))
    }
}
