use axum::http::StatusCode;
use common::BUFFER_SIZE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Body {
    #[default]
    Empty,
    Json(String),
}

impl Body {
    pub async fn try_from(body_in: axum::body::Body) -> Result<Self, StatusCode> {
        let bytes = axum::body::to_bytes(body_in, BUFFER_SIZE)
            .await
            .or(Err(StatusCode::PAYLOAD_TOO_LARGE))?;

        if bytes.is_empty() {
            return Ok(Body::Empty);
        }

        match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(_) => Ok(Body::Json(String::from_utf8_lossy(&bytes).to_string())),
            Err(_) => Err(StatusCode::UNSUPPORTED_MEDIA_TYPE),
        }
    }
}

impl From<Body> for Option<reqwest::Body> {
    fn from(body: Body) -> Self {
        match body {
            Body::Empty => None,
            Body::Json(json) => Some(reqwest::Body::from(json)),
        }
    }
}
