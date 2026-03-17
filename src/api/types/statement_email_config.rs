pub use crate::prelude::*;

/// Configuration for statement email recipients and the sender address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StatementEmailConfig {
    /// The email address from which statements are sent. Always uses a Payabli domain, for example `acme-partners@payabli.com`. If `null`, `noreply@payabli.com` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// List of email addresses that receive billing statements. These are merchant or partner contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}
