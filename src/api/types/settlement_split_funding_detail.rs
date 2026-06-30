pub use crate::prelude::*;

/// A split funding instruction on a settled transaction, enriched with the batch and transfer that paid out the split when that information is available. Returned by the settlement query endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SettlementSplitFundingDetail {
    /// The entrypoint the split was sent to.
    #[serde(rename = "recipientEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_entry_point: Option<String>,
    /// The account the split was sent to.
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A description for the split.
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The amount of the transaction sent to this recipient as a split.
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// The batch number the split was paid out in. Null when the batch isn't available.
    #[serde(rename = "batchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// Identifier of the transfer that carried the split. Null when the transfer isn't available.
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The total amount of the transfer that carried this split.
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub transfer_amount: Option<f64>,
}

impl SettlementSplitFundingDetail {
    pub fn builder() -> SettlementSplitFundingDetailBuilder {
        <SettlementSplitFundingDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementSplitFundingDetailBuilder {
    recipient_entry_point: Option<String>,
    account_id: Option<String>,
    description: Option<String>,
    amount: Option<f64>,
    batch_number: Option<String>,
    transfer_id: Option<i64>,
    transfer_amount: Option<f64>,
}

impl SettlementSplitFundingDetailBuilder {
    pub fn recipient_entry_point(mut self, value: impl Into<String>) -> Self {
        self.recipient_entry_point = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn batch_number(mut self, value: impl Into<String>) -> Self {
        self.batch_number = Some(value.into());
        self
    }

    pub fn transfer_id(mut self, value: i64) -> Self {
        self.transfer_id = Some(value);
        self
    }

    pub fn transfer_amount(mut self, value: f64) -> Self {
        self.transfer_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementSplitFundingDetail`].
    pub fn build(self) -> Result<SettlementSplitFundingDetail, BuildError> {
        Ok(SettlementSplitFundingDetail {
            recipient_entry_point: self.recipient_entry_point,
            account_id: self.account_id,
            description: self.description,
            amount: self.amount,
            batch_number: self.batch_number,
            transfer_id: self.transfer_id,
            transfer_amount: self.transfer_amount,
        })
    }
}
