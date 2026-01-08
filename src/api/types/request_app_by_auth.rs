pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestAppByAuth {
    /// The email address the applicant used to save the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// The referenceId is sent to the applicant via email when they save the application.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}
