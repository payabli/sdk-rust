pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPayoutTransactionSummary {
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(rename = "totalAuthorized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authorized: Option<i64>,
    #[serde(rename = "totalAuthorizedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authorized_amount: Option<f64>,
    #[serde(rename = "totalCanceled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_canceled: Option<i64>,
    #[serde(rename = "totalCanceledAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_canceled_amount: Option<f64>,
    #[serde(rename = "totalCaptured")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_captured: Option<i64>,
    #[serde(rename = "totalCapturedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_captured_amount: Option<f64>,
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    #[serde(rename = "totalOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_open: Option<i64>,
    #[serde(rename = "totalOpenAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_open_amount: Option<f64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "totalPaid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_paid: Option<i64>,
    #[serde(rename = "totalPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_paid_amount: Option<f64>,
    /// Total number of transactions that are currently on hold.
    #[serde(rename = "totalOnHold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_on_hold: Option<i64>,
    /// Total amount of transactions that are currently on hold.
    #[serde(rename = "totalOnHoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_on_hold_amount: Option<f64>,
    #[serde(rename = "totalProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processing: Option<i64>,
    #[serde(rename = "totalProcessingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processing_amount: Option<f64>,
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
}

impl QueryPayoutTransactionSummary {
    pub fn builder() -> QueryPayoutTransactionSummaryBuilder {
        <QueryPayoutTransactionSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPayoutTransactionSummaryBuilder {
    page_identifier: Option<PageIdentifier>,
    page_size: Option<Pagesize>,
    total_amount: Option<f64>,
    total_authorized: Option<i64>,
    total_authorized_amount: Option<f64>,
    total_canceled: Option<i64>,
    total_canceled_amount: Option<f64>,
    total_captured: Option<i64>,
    total_captured_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_open: Option<i64>,
    total_open_amount: Option<f64>,
    total_pages: Option<i64>,
    total_paid: Option<i64>,
    total_paid_amount: Option<f64>,
    total_on_hold: Option<i64>,
    total_on_hold_amount: Option<f64>,
    total_processing: Option<i64>,
    total_processing_amount: Option<f64>,
    total_records: Option<i64>,
}

impl QueryPayoutTransactionSummaryBuilder {
    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn total_authorized(mut self, value: i64) -> Self {
        self.total_authorized = Some(value);
        self
    }

    pub fn total_authorized_amount(mut self, value: f64) -> Self {
        self.total_authorized_amount = Some(value);
        self
    }

    pub fn total_canceled(mut self, value: i64) -> Self {
        self.total_canceled = Some(value);
        self
    }

    pub fn total_canceled_amount(mut self, value: f64) -> Self {
        self.total_canceled_amount = Some(value);
        self
    }

    pub fn total_captured(mut self, value: i64) -> Self {
        self.total_captured = Some(value);
        self
    }

    pub fn total_captured_amount(mut self, value: f64) -> Self {
        self.total_captured_amount = Some(value);
        self
    }

    pub fn total_net_amount(mut self, value: f64) -> Self {
        self.total_net_amount = Some(value);
        self
    }

    pub fn total_open(mut self, value: i64) -> Self {
        self.total_open = Some(value);
        self
    }

    pub fn total_open_amount(mut self, value: f64) -> Self {
        self.total_open_amount = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_paid(mut self, value: i64) -> Self {
        self.total_paid = Some(value);
        self
    }

    pub fn total_paid_amount(mut self, value: f64) -> Self {
        self.total_paid_amount = Some(value);
        self
    }

    pub fn total_on_hold(mut self, value: i64) -> Self {
        self.total_on_hold = Some(value);
        self
    }

    pub fn total_on_hold_amount(mut self, value: f64) -> Self {
        self.total_on_hold_amount = Some(value);
        self
    }

    pub fn total_processing(mut self, value: i64) -> Self {
        self.total_processing = Some(value);
        self
    }

    pub fn total_processing_amount(mut self, value: f64) -> Self {
        self.total_processing_amount = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryPayoutTransactionSummary`].
    pub fn build(self) -> Result<QueryPayoutTransactionSummary, BuildError> {
        Ok(QueryPayoutTransactionSummary {
            page_identifier: self.page_identifier,
            page_size: self.page_size,
            total_amount: self.total_amount,
            total_authorized: self.total_authorized,
            total_authorized_amount: self.total_authorized_amount,
            total_canceled: self.total_canceled,
            total_canceled_amount: self.total_canceled_amount,
            total_captured: self.total_captured,
            total_captured_amount: self.total_captured_amount,
            total_net_amount: self.total_net_amount,
            total_open: self.total_open,
            total_open_amount: self.total_open_amount,
            total_pages: self.total_pages,
            total_paid: self.total_paid,
            total_paid_amount: self.total_paid_amount,
            total_on_hold: self.total_on_hold,
            total_on_hold_amount: self.total_on_hold_amount,
            total_processing: self.total_processing,
            total_processing_amount: self.total_processing_amount,
            total_records: self.total_records,
        })
    }
}
