pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeyValueDuo {
    /// Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
