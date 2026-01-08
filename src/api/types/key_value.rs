pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeyValue {
    /// Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    /// Key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
