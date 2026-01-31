use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
}

impl TryFrom<reqwest::Response> for Response {
    type Error = anyhow::Error;

    fn try_from(_value: reqwest::Response) -> anyhow::Result<Self> {
        Ok(Self::default())
    }
}

impl From<StatusCode> for Response {
    fn from(status_code: StatusCode) -> Self {
        Self { status_code }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        self.status_code.into_response()
    }
}
