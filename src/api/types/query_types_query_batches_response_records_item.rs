pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub batch_amount: Option<f64>,
    /// The total of fees in the batch.
    #[serde(rename = "BatchFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_fees_amount: Option<f64>,
    #[serde(rename = "BatchAuthAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_auth_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review.
    #[serde(rename = "BatchReleasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_released_amount: Option<f64>,
    /// The total amount of the batch that's being held for fraud or risk concerns.
    #[serde(rename = "BatchHoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_hold_amount: Option<f64>,
    /// Total amount of ACH returns deducted from batch.
    #[serde(rename = "BatchReturnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_returned_amount: Option<f64>,
    /// The total amount of refunds deducted from batch.
    #[serde(rename = "BatchRefundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_refund_amount: Option<f64>,
    /// Total of split transactions that included split funding instructions at the time of authorization.
    #[serde(rename = "BatchSplitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_split_amount: Option<f64>,
    /// The batch status. See [Batch Status](/developers/references/money-in-statuses#batch-status) for more.
    #[serde(rename = "BatchStatus")]
    pub batch_status: i64,
    /// The number of records in the batch.
    #[serde(rename = "BatchRecords")]
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
    pub parent_org_name: OrgParentName,
    /// The parent organization ID.
    #[serde(rename = "ParentOrgId")]
    pub parent_org_id: i64,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "EntryName")]
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
