pub use crate::prelude::*;

/// Configuration for statement email recipients and the sender address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementEmailConfig {
    /// The email address from which statements are sent. Always uses a Payabli domain, for example `acme-partners@payabli.com`. If `null`, `noreply@payabli.com` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// List of email addresses that receive billing statements. These are merchant or partner contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
}

impl StatementEmailConfig {
    pub fn builder() -> StatementEmailConfigBuilder {
        <StatementEmailConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementEmailConfigBuilder {
    sender: Option<String>,
    recipients: Option<Vec<String>>,
}

impl StatementEmailConfigBuilder {
    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementEmailConfig`].
    pub fn build(self) -> Result<StatementEmailConfig, BuildError> {
        Ok(StatementEmailConfig {
            sender: self.sender,
            recipients: self.recipients,
        })
    }
}
