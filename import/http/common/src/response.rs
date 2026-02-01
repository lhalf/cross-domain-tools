use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
}

impl TryFrom<reqwest::Response> for Response {
    type Error = anyhow::Error;

    fn try_from(response: reqwest::Response) -> anyhow::Result<Self> {
        Ok(Self {
            status_code: response.status(),
            headers: response.headers().clone(),
        })
    }
}

impl From<StatusCode> for Response {
    fn from(status_code: StatusCode) -> Self {
        Self {
            status_code,
            headers: HeaderMap::default(),
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.headers).into_response()
    }
}
