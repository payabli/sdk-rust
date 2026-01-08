pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryBatchesOutResponseRecordsItem {
    #[serde(rename = "AchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_amount: Option<f64>,
    #[serde(rename = "AchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_records: Option<i64>,
    #[serde(rename = "AchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_status: Option<i64>,
    #[serde(rename = "AchStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_status_text: Option<String>,
    /// The amount of the batch.
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_amount: Option<f64>,
    #[serde(rename = "BatchCancelledAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_cancelled_amount: Option<f64>,
    #[serde(rename = "BatchCancelledRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_cancelled_records: Option<i64>,
    /// The batch date.
    #[serde(rename = "BatchDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub batch_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    #[serde(rename = "BatchPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_paid_amount: Option<f64>,
    #[serde(rename = "BatchPaidRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_paid_records: Option<i64>,
    #[serde(rename = "BatchProcessedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_processed_amount: Option<f64>,
    #[serde(rename = "BatchProcessedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_processed_records: Option<i64>,
    #[serde(rename = "BatchProcessingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_processing_amount: Option<f64>,
    #[serde(rename = "BatchProcessingRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_processing_records: Option<i64>,
    /// The number of records in the batch.
    #[serde(rename = "BatchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_records: Option<i64>,
    /// The batch status. See [Batch Status](/developers/references/money-out-statuses#batch-statuses) for more.
    #[serde(rename = "BatchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<i64>,
    /// A text description of the batch status.
    #[serde(rename = "BatchStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_status_text: Option<String>,
    #[serde(rename = "CardAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_amount: Option<f64>,
    #[serde(rename = "CardRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_records: Option<i64>,
    #[serde(rename = "CardStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status: Option<i64>,
    #[serde(rename = "CardStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_status_text: Option<String>,
    #[serde(rename = "CheckAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<f64>,
    #[serde(rename = "CheckRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_records: Option<i64>,
    #[serde(rename = "CheckStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_status: Option<i64>,
    #[serde(rename = "CheckStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_status_text: Option<String>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<Entrypointfield>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// The batch ID.
    #[serde(rename = "IdBatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_batch: Option<i64>,
    /// The entrypoint's parent org.
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// Paypoint DBA name.
    #[serde(rename = "PaypointDba")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba: Option<String>,
    /// Paypoint ID.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// Paypoint legal name.
    #[serde(rename = "PaypointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_name: Option<String>,
    #[serde(rename = "VcardAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_amount: Option<f64>,
    #[serde(rename = "VcardRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_records: Option<i64>,
    #[serde(rename = "VcardStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_status: Option<i64>,
    #[serde(rename = "VcardStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_status_text: Option<String>,
    #[serde(rename = "WireAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_amount: Option<f64>,
    #[serde(rename = "WireRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_records: Option<i64>,
    #[serde(rename = "WireStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_status: Option<i64>,
    #[serde(rename = "WireStatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wire_status_text: Option<String>,
}
