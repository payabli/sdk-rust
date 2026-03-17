pub use crate::prelude::*;

/// Details about the Google Pay service status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GooglePayStatusData {
    /// Any error message related to Google Pay's activation status.
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GooglePayMetadata>,
}
