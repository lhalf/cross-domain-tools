use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Options,
    Patch,
    Trace,
}

impl TryFrom<&reqwest::Method> for Method {
    type Error = anyhow::Error;

    fn try_from(method: &axum::http::Method) -> Result<Self, Self::Error> {
        use axum::http::Method;

        match *method {
            Method::GET => Ok(Self::Get),
            Method::POST => Ok(Self::Post),
            Method::PUT => Ok(Self::Put),
            Method::DELETE => Ok(Self::Delete),
            Method::OPTIONS => Ok(Self::Options),
            Method::PATCH => Ok(Self::Patch),
            Method::TRACE => Ok(Self::Trace),
            _ => Err(anyhow::anyhow!("unsupported method")),
        }
    }
}

impl From<Method> for axum::http::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => Self::GET,
            Method::Post => Self::POST,
            Method::Put => Self::PUT,
            Method::Delete => Self::DELETE,
            Method::Options => Self::OPTIONS,
            Method::Patch => Self::PATCH,
            Method::Trace => Self::TRACE,
        }
    }
}
