use crate::method::Method;
use axum::http::StatusCode;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

impl<S: Sync> axum::extract::FromRequest<S> for Request {
    type Rejection = StatusCode;

    async fn from_request(
        request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Request {
            method: request
                .method()
                .try_into()
                .map_err(|_| StatusCode::METHOD_NOT_ALLOWED)?,
            path: request.uri().path().to_string(),
        })
    }
}

impl TryFrom<Request> for reqwest::Request {
    type Error = anyhow::Error;

    fn try_from(request: Request) -> anyhow::Result<Self> {
        // TODO: make the target destination a config variable
        Ok(Self::new(
            request.method.into(),
            Url::parse(&format!("http://localhost:9002{}", request.path))?,
        ))
    }
}
