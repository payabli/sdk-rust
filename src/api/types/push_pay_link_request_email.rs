pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PushPayLinkRequestEmail {
    /// List of additional email addresses you want to send the paylink to, formatted as an array.
    /// Payment links and opt-in requests are sent to the customer email address on file, and additional
    /// recipients can be specified here.
    #[serde(rename = "additionalEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_emails: Option<Vec<String>>,
    /// When `true`, attaches a PDF version of the invoice to the email.
    #[serde(rename = "attachFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_file: Option<bool>,
}
