pub use crate::prelude::*;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
