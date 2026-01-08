pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChargeBackResponse {
    /// Object with attached files to response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<BoardingApplicationAttachments>,
    /// Email of response submitter.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Name of response submitter
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Timestamp when response was submitted, in UTC.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Chargeback response identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Response notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
