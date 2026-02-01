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

impl TryFrom<Request> for reqwest::Request {
    type Error = anyhow::Error;

    fn try_from(request_in: Request) -> anyhow::Result<Self> {
        // TODO: make the target destination a config variable
        let mut url = Url::parse(&format!("http://localhost:9002{}", request_in.path))?;

        url.set_query(request_in.query.as_deref());

        let mut request_out = reqwest::Request::new(request_in.method, url);
        *request_out.headers_mut() = request_in.headers;

        Ok(request_out)
    }
}
