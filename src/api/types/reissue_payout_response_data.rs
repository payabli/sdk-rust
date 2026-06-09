pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissuePayoutResponseData {
    /// The transaction ID of the newly created payout.
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    /// The status of the new transaction.
    #[serde(default)]
    pub status: String,
    /// The transaction ID of the original payout that was reissued.
    #[serde(rename = "originalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction_id: Option<String>,
}

impl ReissuePayoutResponseData {
    pub fn builder() -> ReissuePayoutResponseDataBuilder {
        <ReissuePayoutResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissuePayoutResponseDataBuilder {
    transaction_id: Option<String>,
    status: Option<String>,
    original_transaction_id: Option<String>,
}

impl ReissuePayoutResponseDataBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn original_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.original_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReissuePayoutResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](ReissuePayoutResponseDataBuilder::transaction_id)
    /// - [`status`](ReissuePayoutResponseDataBuilder::status)
    pub fn build(self) -> Result<ReissuePayoutResponseData, BuildError> {
        Ok(ReissuePayoutResponseData {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            original_transaction_id: self.original_transaction_id,
        })
    }
}
