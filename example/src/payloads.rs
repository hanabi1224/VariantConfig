use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertPayload {
    pub variants: serde_json::Value,
    pub content: String,
    pub type_: String,
}
