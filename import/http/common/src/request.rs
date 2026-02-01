use axum::http::{HeaderMap, Method, StatusCode};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;

#[derive(Serialize, Deserialize, Default)]
pub struct Request {
    #[serde(with = "http_serde::method")]
    pub method: Method,
    pub path: String,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
    pub query: Option<String>,
}

impl<S: Sync> axum::extract::FromRequest<S> for Request {
    type Rejection = StatusCode;

    async fn from_request(
        request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            method: request.method().clone(),
            path: request.uri().path().to_string(),
            headers: request.headers().clone(),
            query: request.uri().query().map(ToString::to_string),
        })
    }
}

impl Request {
    pub fn try_to_reqwest(self, target_address: SocketAddrV4) -> anyhow::Result<reqwest::Request> {
        let mut url = Url::parse(&format!("http://{}{}", target_address, self.path))?;

        url.set_query(self.query.as_deref());

        let mut request_out = reqwest::Request::new(self.method, url);
        *request_out.headers_mut() = self.headers;

        Ok(request_out)
    }
}
