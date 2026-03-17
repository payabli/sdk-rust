pub use crate::prelude::*;

/// Describes the reason for a failed operation and how to resolve it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseError400ResponseData {
    /// Describes the reason the operation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Describes how to resolve the error.
    #[serde(rename = "todoAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_action: Option<String>,
}
