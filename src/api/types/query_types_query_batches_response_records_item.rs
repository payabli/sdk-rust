pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBatchesResponseRecordsItem {
    /// The batch ID.
    #[serde(rename = "IdBatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_batch: Option<i64>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    #[serde(rename = "TransferIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_identifier: Option<TransferIdentifier>,
    /// Events associated with the batch.
    #[serde(rename = "EventsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_data: Option<Vec<GeneralEvents>>,
    #[serde(rename = "ConnectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    /// The batch date.
    #[serde(rename = "BatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub batch_date: Option<DateTime<Utc>>,
    /// The amount of the batch.
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_amount: Option<f64>,
    /// The total of fees in the batch.
    #[serde(rename = "BatchFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_fees_amount: Option<f64>,
    #[serde(rename = "BatchAuthAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_auth_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review.
    #[serde(rename = "BatchReleasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_released_amount: Option<f64>,
    /// The total amount of the batch that's being held for fraud or risk concerns.
    #[serde(rename = "BatchHoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_hold_amount: Option<f64>,
    /// Total amount of ACH returns deducted from batch.
    #[serde(rename = "BatchReturnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_returned_amount: Option<f64>,
    /// The total amount of refunds deducted from batch.
    #[serde(rename = "BatchRefundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_refund_amount: Option<f64>,
    /// Total of split transactions that included split funding instructions at the time of authorization.
    #[serde(rename = "BatchSplitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_split_amount: Option<f64>,
    /// The batch status. See [Batch Status](/developers/references/money-in-statuses#batch-status) for more.
    #[serde(rename = "BatchStatus")]
    #[serde(default)]
    pub batch_status: i64,
    /// The number of records in the batch.
    #[serde(rename = "BatchRecords")]
    #[serde(default)]
    pub batch_records: i64,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    #[serde(rename = "PaypointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_name: Option<PaypointName>,
    #[serde(rename = "PaypointDba")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba: Option<Dbaname>,
    /// The entrypoint's parent org.
    #[serde(rename = "ParentOrgName")]
    #[serde(default)]
    pub parent_org_name: OrgParentName,
    /// The parent organization ID.
    #[serde(rename = "ParentOrgId")]
    #[serde(default)]
    pub parent_org_id: i64,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "EntryName")]
    #[serde(default)]
    pub entry_name: Entrypointfield,
    /// The bank name.
    #[serde(rename = "BankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The batch type.
    #[serde(rename = "BatchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_type: Option<i64>,
    /// The payment method used.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "ExpectedDepositDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_deposit_date: Option<ExpectedDepositDate>,
    #[serde(rename = "DepositDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_date: Option<DepositDate>,
    /// The batch transfer date.
    #[serde(rename = "TransferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transfer_date: Option<DateTime<Utc>>,
    /// Transfer details for the batch.
    #[serde(rename = "Transfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<QueryBatchesTransfer>,
}

impl QueryBatchesResponseRecordsItem {
    pub fn builder() -> QueryBatchesResponseRecordsItemBuilder {
        <QueryBatchesResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesResponseRecordsItemBuilder {
    id_batch: Option<i64>,
    batch_number: Option<BatchNumber>,
    transfer_identifier: Option<TransferIdentifier>,
    events_data: Option<Vec<GeneralEvents>>,
    connector_name: Option<String>,
    batch_date: Option<DateTime<Utc>>,
    batch_amount: Option<f64>,
    batch_fees_amount: Option<f64>,
    batch_auth_amount: Option<f64>,
    batch_released_amount: Option<f64>,
    batch_hold_amount: Option<f64>,
    batch_returned_amount: Option<f64>,
    batch_refund_amount: Option<f64>,
    batch_split_amount: Option<f64>,
    batch_status: Option<i64>,
    batch_records: Option<i64>,
    paypoint_id: Option<PaypointId>,
    paypoint_name: Option<PaypointName>,
    paypoint_dba: Option<Dbaname>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<i64>,
    external_paypoint_id: Option<ExternalPaypointId>,
    entry_name: Option<Entrypointfield>,
    bank_name: Option<String>,
    batch_type: Option<i64>,
    method: Option<String>,
    expected_deposit_date: Option<ExpectedDepositDate>,
    deposit_date: Option<DepositDate>,
    transfer_date: Option<DateTime<Utc>>,
    transfer: Option<QueryBatchesTransfer>,
}

impl QueryBatchesResponseRecordsItemBuilder {
    pub fn id_batch(mut self, value: i64) -> Self {
        self.id_batch = Some(value);
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn transfer_identifier(mut self, value: TransferIdentifier) -> Self {
        self.transfer_identifier = Some(value);
        self
    }

    pub fn events_data(mut self, value: Vec<GeneralEvents>) -> Self {
        self.events_data = Some(value);
        self
    }

    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
        self.connector_name = Some(value.into());
        self
    }

    pub fn batch_date(mut self, value: DateTime<Utc>) -> Self {
        self.batch_date = Some(value);
        self
    }

    pub fn batch_amount(mut self, value: f64) -> Self {
        self.batch_amount = Some(value);
        self
    }

    pub fn batch_fees_amount(mut self, value: f64) -> Self {
        self.batch_fees_amount = Some(value);
        self
    }

    pub fn batch_auth_amount(mut self, value: f64) -> Self {
        self.batch_auth_amount = Some(value);
        self
    }

    pub fn batch_released_amount(mut self, value: f64) -> Self {
        self.batch_released_amount = Some(value);
        self
    }

    pub fn batch_hold_amount(mut self, value: f64) -> Self {
        self.batch_hold_amount = Some(value);
        self
    }

    pub fn batch_returned_amount(mut self, value: f64) -> Self {
        self.batch_returned_amount = Some(value);
        self
    }

    pub fn batch_refund_amount(mut self, value: f64) -> Self {
        self.batch_refund_amount = Some(value);
        self
    }

    pub fn batch_split_amount(mut self, value: f64) -> Self {
        self.batch_split_amount = Some(value);
        self
    }

    pub fn batch_status(mut self, value: i64) -> Self {
        self.batch_status = Some(value);
        self
    }

    pub fn batch_records(mut self, value: i64) -> Self {
        self.batch_records = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_name(mut self, value: PaypointName) -> Self {
        self.paypoint_name = Some(value);
        self
    }

    pub fn paypoint_dba(mut self, value: Dbaname) -> Self {
        self.paypoint_dba = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn entry_name(mut self, value: Entrypointfield) -> Self {
        self.entry_name = Some(value);
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn batch_type(mut self, value: i64) -> Self {
        self.batch_type = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn expected_deposit_date(mut self, value: ExpectedDepositDate) -> Self {
        self.expected_deposit_date = Some(value);
        self
    }

    pub fn deposit_date(mut self, value: DepositDate) -> Self {
        self.deposit_date = Some(value);
        self
    }

    pub fn transfer_date(mut self, value: DateTime<Utc>) -> Self {
        self.transfer_date = Some(value);
        self
    }

    pub fn transfer(mut self, value: QueryBatchesTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesResponseRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_status`](QueryBatchesResponseRecordsItemBuilder::batch_status)
    /// - [`batch_records`](QueryBatchesResponseRecordsItemBuilder::batch_records)
    /// - [`parent_org_name`](QueryBatchesResponseRecordsItemBuilder::parent_org_name)
    /// - [`parent_org_id`](QueryBatchesResponseRecordsItemBuilder::parent_org_id)
    /// - [`entry_name`](QueryBatchesResponseRecordsItemBuilder::entry_name)
    pub fn build(self) -> Result<QueryBatchesResponseRecordsItem, BuildError> {
        Ok(QueryBatchesResponseRecordsItem {
            id_batch: self.id_batch,
            batch_number: self.batch_number,
            transfer_identifier: self.transfer_identifier,
            events_data: self.events_data,
            connector_name: self.connector_name,
            batch_date: self.batch_date,
            batch_amount: self.batch_amount,
            batch_fees_amount: self.batch_fees_amount,
            batch_auth_amount: self.batch_auth_amount,
            batch_released_amount: self.batch_released_amount,
            batch_hold_amount: self.batch_hold_amount,
            batch_returned_amount: self.batch_returned_amount,
            batch_refund_amount: self.batch_refund_amount,
            batch_split_amount: self.batch_split_amount,
            batch_status: self
                .batch_status
                .ok_or_else(|| BuildError::missing_field("batch_status"))?,
            batch_records: self
                .batch_records
                .ok_or_else(|| BuildError::missing_field("batch_records"))?,
            paypoint_id: self.paypoint_id,
            paypoint_name: self.paypoint_name,
            paypoint_dba: self.paypoint_dba,
            parent_org_name: self
                .parent_org_name
                .ok_or_else(|| BuildError::missing_field("parent_org_name"))?,
            parent_org_id: self
                .parent_org_id
                .ok_or_else(|| BuildError::missing_field("parent_org_id"))?,
            external_paypoint_id: self.external_paypoint_id,
            entry_name: self
                .entry_name
                .ok_or_else(|| BuildError::missing_field("entry_name"))?,
            bank_name: self.bank_name,
            batch_type: self.batch_type,
            method: self.method,
            expected_deposit_date: self.expected_deposit_date,
            deposit_date: self.deposit_date,
            transfer_date: self.transfer_date,
            transfer: self.transfer,
        })
    }
}
