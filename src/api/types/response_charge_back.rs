pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponseChargeBack {
    /// Array of attached files to response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    /// Email of response submitter.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Name of response submitter
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Response notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
