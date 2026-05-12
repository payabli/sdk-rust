pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SplitFundingRefundContent {
    /// The accountId for the account the transaction was routed to.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The amount to refund to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Refund description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The entrypoint the transaction belongs to.
    #[serde(rename = "originationEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_entry_point: Option<String>,
}

impl SplitFundingRefundContent {
    pub fn builder() -> SplitFundingRefundContentBuilder {
        <SplitFundingRefundContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SplitFundingRefundContentBuilder {
    account_id: Option<String>,
    amount: Option<f64>,
    description: Option<String>,
    origination_entry_point: Option<String>,
}

impl SplitFundingRefundContentBuilder {
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

    pub fn origination_entry_point(mut self, value: impl Into<String>) -> Self {
        self.origination_entry_point = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SplitFundingRefundContent`].
    pub fn build(self) -> Result<SplitFundingRefundContent, BuildError> {
        Ok(SplitFundingRefundContent {
            account_id: self.account_id,
            amount: self.amount,
            description: self.description,
            origination_entry_point: self.origination_entry_point,
        })
    }
}
