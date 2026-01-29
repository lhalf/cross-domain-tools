use crate::request::Request;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "test", derive(Default))]
#[derive(Serialize, Deserialize)]
pub struct ImportPayload {
    pub uuid: Uuid,
    pub request: Request,
}

#[cfg_attr(feature = "test", derive(Default))]
#[derive(Serialize, Deserialize)]
pub struct ExportPayload {
    pub uuid: Uuid,
    pub response: Response,
}
