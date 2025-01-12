use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AckResponse {
    pub(crate) method: String,
    pub(crate) success: bool,
    pub(crate) result: Option<Map<String, Value>>,
    pub(crate) error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataResponse {
    pub(crate) channel: String,
    pub(crate) r#type: String,
    pub(crate) data: Vec<Value>,
}
