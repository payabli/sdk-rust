pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAuthResetRequest {
    /// The user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Identifier for entrypoint originating the request (used by front-end apps)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Type of entry identifier: 0 - partner, 2 - paypoint. This is used by front-end apps, required if an Entry is indicated.
    #[serde(rename = "entryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<i64>,
}
