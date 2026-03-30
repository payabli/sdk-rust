pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SplitFundingContent {
    /// The accountId for the account the split should be sent to.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Amount from the transaction to send to this recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// A description for the split.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The entrypoint the split should be sent to.
    #[serde(rename = "recipientEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_entry_point: Option<String>,
}

impl SplitFundingContent {
    pub fn builder() -> SplitFundingContentBuilder {
        <SplitFundingContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SplitFundingContentBuilder {
    account_id: Option<String>,
    amount: Option<f64>,
    description: Option<String>,
    recipient_entry_point: Option<String>,
}

impl SplitFundingContentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn recipient_entry_point(mut self, value: impl Into<String>) -> Self {
        self.recipient_entry_point = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SplitFundingContent`].
    pub fn build(self) -> Result<SplitFundingContent, BuildError> {
        Ok(SplitFundingContent {
            account_id: self.account_id,
            amount: self.amount,
            description: self.description,
            recipient_entry_point: self.recipient_entry_point,
        })
    }
}
