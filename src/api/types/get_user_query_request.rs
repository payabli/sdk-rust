pub use crate::prelude::*;

/// Query parameters for GetUser
///
/// Request type for the GetUserQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserQueryRequest {
    /// The entrypoint identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Entry level: 0 - partner, 2 - paypoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}
