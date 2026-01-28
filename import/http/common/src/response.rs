use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
}

impl TryFrom<reqwest::Response> for Response {
    type Error = anyhow::Error;

    fn try_from(_value: reqwest::Response) -> anyhow::Result<Self> {
        Ok(Response::default())
    }
}
