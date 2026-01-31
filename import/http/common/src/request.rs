use axum::http::{HeaderMap, Method, StatusCode};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Request {
    #[serde(with = "http_serde::method")]
    pub method: Method,
    pub path: String,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
}

impl<S: Sync> axum::extract::FromRequest<S> for Request {
    type Rejection = StatusCode;

    async fn from_request(
        request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Request {
            method: request.method().clone(),
            path: request.uri().path().to_string(),
            headers: request.headers().clone(),
        })
    }
}

impl TryFrom<Request> for reqwest::Request {
    type Error = anyhow::Error;

    fn try_from(request: Request) -> anyhow::Result<Self> {
        // TODO: make the target destination a config variable
        let mut reqwest = Self::new(
            request.method,
            Url::parse(&format!("http://localhost:9002{}", request.path))?,
        );

        *reqwest.headers_mut() = request.headers;

        Ok(reqwest)
    }
}
