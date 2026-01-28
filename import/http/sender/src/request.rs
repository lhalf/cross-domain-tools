use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Request;

impl<S: Sync> axum::extract::FromRequest<S> for Request {
    type Rejection = StatusCode;

    async fn from_request(
        _request: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Request)
    }
}
