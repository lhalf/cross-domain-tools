use crate::request::Request;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ImportPayload {
    uuid: Uuid,
    request: Request,
}

impl ImportPayload {
    pub fn new(uuid: Uuid, request: Request) -> Self {
        Self { uuid, request }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExportPayload {
    uuid: Uuid,
    response: Response,
}
