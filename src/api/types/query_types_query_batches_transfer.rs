pub use crate::prelude::*;

/// Transfer details within a batch response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBatchesTransfer {
    /// The transfer ID.
    #[serde(rename = "TransferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The transfer date.
    #[serde(rename = "TransferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transfer_date: Option<DateTime<Utc>>,
    /// The processor used for the transfer.
    #[serde(rename = "Processor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
    /// The transfer status.
    #[serde(rename = "TransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<i64>,
    /// The gross amount of the transfer.
    #[serde(rename = "GrossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    /// The chargeback amount.
    #[serde(rename = "ChargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_back_amount: Option<f64>,
    /// The returned amount.
    #[serde(rename = "ReturnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_amount: Option<f64>,
    /// The refund amount.
    #[serde(rename = "RefundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<f64>,
    /// The amount being held.
    #[serde(rename = "HoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_amount: Option<f64>,
    /// The amount that has been released.
    #[serde(rename = "ReleasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_amount: Option<f64>,
    /// The billing fees amount.
    #[serde(rename = "BillingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_amount: Option<f64>,
    /// The third party paid amount.
    #[serde(rename = "ThirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid_amount: Option<f64>,
    /// The adjustments amount.
    #[serde(rename = "AdjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments_amount: Option<f64>,
    /// The net funded amount.
    #[serde(rename = "NetFundedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_funded_amount: Option<f64>,
}

impl QueryBatchesTransfer {
    pub fn builder() -> QueryBatchesTransferBuilder {
        <QueryBatchesTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesTransferBuilder {
    transfer_id: Option<i64>,
    transfer_date: Option<DateTime<Utc>>,
    processor: Option<String>,
    transfer_status: Option<i64>,
    gross_amount: Option<f64>,
    charge_back_amount: Option<f64>,
    returned_amount: Option<f64>,
    refund_amount: Option<f64>,
    hold_amount: Option<f64>,
    released_amount: Option<f64>,
    billing_fees_amount: Option<f64>,
    third_party_paid_amount: Option<f64>,
    adjustments_amount: Option<f64>,
    net_funded_amount: Option<f64>,
}

impl QueryBatchesTransferBuilder {
    pub fn transfer_id(mut self, value: i64) -> Self {
        self.transfer_id = Some(value);
        self
    }

    pub fn transfer_date(mut self, value: DateTime<Utc>) -> Self {
        self.transfer_date = Some(value);
        self
    }

    pub fn processor(mut self, value: impl Into<String>) -> Self {
        self.processor = Some(value.into());
        self
    }

    pub fn transfer_status(mut self, value: i64) -> Self {
        self.transfer_status = Some(value);
        self
    }

    pub fn gross_amount(mut self, value: f64) -> Self {
        self.gross_amount = Some(value);
        self
    }

    pub fn charge_back_amount(mut self, value: f64) -> Self {
        self.charge_back_amount = Some(value);
        self
    }

    pub fn returned_amount(mut self, value: f64) -> Self {
        self.returned_amount = Some(value);
        self
    }

    pub fn refund_amount(mut self, value: f64) -> Self {
        self.refund_amount = Some(value);
        self
    }

    pub fn hold_amount(mut self, value: f64) -> Self {
        self.hold_amount = Some(value);
        self
    }

    pub fn released_amount(mut self, value: f64) -> Self {
        self.released_amount = Some(value);
        self
    }

    pub fn billing_fees_amount(mut self, value: f64) -> Self {
        self.billing_fees_amount = Some(value);
        self
    }

    pub fn third_party_paid_amount(mut self, value: f64) -> Self {
        self.third_party_paid_amount = Some(value);
        self
    }

    pub fn adjustments_amount(mut self, value: f64) -> Self {
        self.adjustments_amount = Some(value);
        self
    }

    pub fn net_funded_amount(mut self, value: f64) -> Self {
        self.net_funded_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesTransfer`].
    pub fn build(self) -> Result<QueryBatchesTransfer, BuildError> {
        Ok(QueryBatchesTransfer {
            transfer_id: self.transfer_id,
            transfer_date: self.transfer_date,
            processor: self.processor,
            transfer_status: self.transfer_status,
            gross_amount: self.gross_amount,
            charge_back_amount: self.charge_back_amount,
            returned_amount: self.returned_amount,
            refund_amount: self.refund_amount,
            hold_amount: self.hold_amount,
            released_amount: self.released_amount,
            billing_fees_amount: self.billing_fees_amount,
            third_party_paid_amount: self.third_party_paid_amount,
            adjustments_amount: self.adjustments_amount,
            net_funded_amount: self.net_funded_amount,
        })
    }
}
