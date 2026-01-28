use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Response;

impl TryFrom<reqwest::Response> for Response {
    type Error = anyhow::Error;

    fn try_from(_value: reqwest::Response) -> anyhow::Result<Self> {
        Ok(Response)
    }
}
