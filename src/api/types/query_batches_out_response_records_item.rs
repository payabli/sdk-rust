pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub batch_date: Option<DateTime<Utc>>,
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

impl QueryBatchesOutResponseRecordsItem {
    pub fn builder() -> QueryBatchesOutResponseRecordsItemBuilder {
        <QueryBatchesOutResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesOutResponseRecordsItemBuilder {
    ach_amount: Option<f64>,
    ach_records: Option<i64>,
    ach_status: Option<i64>,
    ach_status_text: Option<String>,
    batch_amount: Option<f64>,
    batch_cancelled_amount: Option<f64>,
    batch_cancelled_records: Option<i64>,
    batch_date: Option<DateTime<Utc>>,
    batch_number: Option<BatchNumber>,
    batch_paid_amount: Option<f64>,
    batch_paid_records: Option<i64>,
    batch_processed_amount: Option<f64>,
    batch_processed_records: Option<i64>,
    batch_processing_amount: Option<f64>,
    batch_processing_records: Option<i64>,
    batch_records: Option<i64>,
    batch_status: Option<i64>,
    batch_status_text: Option<String>,
    card_amount: Option<f64>,
    card_records: Option<i64>,
    card_status: Option<i64>,
    card_status_text: Option<String>,
    check_amount: Option<f64>,
    check_records: Option<i64>,
    check_status: Option<i64>,
    check_status_text: Option<String>,
    entry_name: Option<Entrypointfield>,
    external_paypoint_id: Option<ExternalPaypointId>,
    id_batch: Option<i64>,
    parent_org_name: Option<String>,
    paypoint_dba: Option<String>,
    paypoint_id: Option<i64>,
    paypoint_name: Option<String>,
    vcard_amount: Option<f64>,
    vcard_records: Option<i64>,
    vcard_status: Option<i64>,
    vcard_status_text: Option<String>,
    wire_amount: Option<f64>,
    wire_records: Option<i64>,
    wire_status: Option<i64>,
    wire_status_text: Option<String>,
}

impl QueryBatchesOutResponseRecordsItemBuilder {
    pub fn ach_amount(mut self, value: f64) -> Self {
        self.ach_amount = Some(value);
        self
    }

    pub fn ach_records(mut self, value: i64) -> Self {
        self.ach_records = Some(value);
        self
    }

    pub fn ach_status(mut self, value: i64) -> Self {
        self.ach_status = Some(value);
        self
    }

    pub fn ach_status_text(mut self, value: impl Into<String>) -> Self {
        self.ach_status_text = Some(value.into());
        self
    }

    pub fn batch_amount(mut self, value: f64) -> Self {
        self.batch_amount = Some(value);
        self
    }

    pub fn batch_cancelled_amount(mut self, value: f64) -> Self {
        self.batch_cancelled_amount = Some(value);
        self
    }

    pub fn batch_cancelled_records(mut self, value: i64) -> Self {
        self.batch_cancelled_records = Some(value);
        self
    }

    pub fn batch_date(mut self, value: DateTime<Utc>) -> Self {
        self.batch_date = Some(value);
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn batch_paid_amount(mut self, value: f64) -> Self {
        self.batch_paid_amount = Some(value);
        self
    }

    pub fn batch_paid_records(mut self, value: i64) -> Self {
        self.batch_paid_records = Some(value);
        self
    }

    pub fn batch_processed_amount(mut self, value: f64) -> Self {
        self.batch_processed_amount = Some(value);
        self
    }

    pub fn batch_processed_records(mut self, value: i64) -> Self {
        self.batch_processed_records = Some(value);
        self
    }

    pub fn batch_processing_amount(mut self, value: f64) -> Self {
        self.batch_processing_amount = Some(value);
        self
    }

    pub fn batch_processing_records(mut self, value: i64) -> Self {
        self.batch_processing_records = Some(value);
        self
    }

    pub fn batch_records(mut self, value: i64) -> Self {
        self.batch_records = Some(value);
        self
    }

    pub fn batch_status(mut self, value: i64) -> Self {
        self.batch_status = Some(value);
        self
    }

    pub fn batch_status_text(mut self, value: impl Into<String>) -> Self {
        self.batch_status_text = Some(value.into());
        self
    }

    pub fn card_amount(mut self, value: f64) -> Self {
        self.card_amount = Some(value);
        self
    }

    pub fn card_records(mut self, value: i64) -> Self {
        self.card_records = Some(value);
        self
    }

    pub fn card_status(mut self, value: i64) -> Self {
        self.card_status = Some(value);
        self
    }

    pub fn card_status_text(mut self, value: impl Into<String>) -> Self {
        self.card_status_text = Some(value.into());
        self
    }

    pub fn check_amount(mut self, value: f64) -> Self {
        self.check_amount = Some(value);
        self
    }

    pub fn check_records(mut self, value: i64) -> Self {
        self.check_records = Some(value);
        self
    }

    pub fn check_status(mut self, value: i64) -> Self {
        self.check_status = Some(value);
        self
    }

    pub fn check_status_text(mut self, value: impl Into<String>) -> Self {
        self.check_status_text = Some(value.into());
        self
    }

    pub fn entry_name(mut self, value: Entrypointfield) -> Self {
        self.entry_name = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn id_batch(mut self, value: i64) -> Self {
        self.id_batch = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn paypoint_dba(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dba = Some(value.into());
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_name = Some(value.into());
        self
    }

    pub fn vcard_amount(mut self, value: f64) -> Self {
        self.vcard_amount = Some(value);
        self
    }

    pub fn vcard_records(mut self, value: i64) -> Self {
        self.vcard_records = Some(value);
        self
    }

    pub fn vcard_status(mut self, value: i64) -> Self {
        self.vcard_status = Some(value);
        self
    }

    pub fn vcard_status_text(mut self, value: impl Into<String>) -> Self {
        self.vcard_status_text = Some(value.into());
        self
    }

    pub fn wire_amount(mut self, value: f64) -> Self {
        self.wire_amount = Some(value);
        self
    }

    pub fn wire_records(mut self, value: i64) -> Self {
        self.wire_records = Some(value);
        self
    }

    pub fn wire_status(mut self, value: i64) -> Self {
        self.wire_status = Some(value);
        self
    }

    pub fn wire_status_text(mut self, value: impl Into<String>) -> Self {
        self.wire_status_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesOutResponseRecordsItem`].
    pub fn build(self) -> Result<QueryBatchesOutResponseRecordsItem, BuildError> {
        Ok(QueryBatchesOutResponseRecordsItem {
            ach_amount: self.ach_amount,
            ach_records: self.ach_records,
            ach_status: self.ach_status,
            ach_status_text: self.ach_status_text,
            batch_amount: self.batch_amount,
            batch_cancelled_amount: self.batch_cancelled_amount,
            batch_cancelled_records: self.batch_cancelled_records,
            batch_date: self.batch_date,
            batch_number: self.batch_number,
            batch_paid_amount: self.batch_paid_amount,
            batch_paid_records: self.batch_paid_records,
            batch_processed_amount: self.batch_processed_amount,
            batch_processed_records: self.batch_processed_records,
            batch_processing_amount: self.batch_processing_amount,
            batch_processing_records: self.batch_processing_records,
            batch_records: self.batch_records,
            batch_status: self.batch_status,
            batch_status_text: self.batch_status_text,
            card_amount: self.card_amount,
            card_records: self.card_records,
            card_status: self.card_status,
            card_status_text: self.card_status_text,
            check_amount: self.check_amount,
            check_records: self.check_records,
            check_status: self.check_status,
            check_status_text: self.check_status_text,
            entry_name: self.entry_name,
            external_paypoint_id: self.external_paypoint_id,
            id_batch: self.id_batch,
            parent_org_name: self.parent_org_name,
            paypoint_dba: self.paypoint_dba,
            paypoint_id: self.paypoint_id,
            paypoint_name: self.paypoint_name,
            vcard_amount: self.vcard_amount,
            vcard_records: self.vcard_records,
            vcard_status: self.vcard_status,
            vcard_status_text: self.vcard_status_text,
            wire_amount: self.wire_amount,
            wire_records: self.wire_records,
            wire_status: self.wire_status,
            wire_status_text: self.wire_status_text,
        })
    }
}
