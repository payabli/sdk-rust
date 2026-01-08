pub use crate::prelude::*;

/// Query parameters for sendPayLinkFromId
///
/// Request type for the SendPayLinkFromIdQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendPayLinkFromIdQueryRequest {
    /// When `true`, attaches a PDF version of invoice to the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachfile: Option<bool>,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
}
