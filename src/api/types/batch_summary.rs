pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Number of records on each response page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Total amount for the records.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Total net amount for the records.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Total number of pages in response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Total number of records in response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
}

impl BatchSummary {
    pub fn builder() -> BatchSummaryBuilder {
        <BatchSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchSummaryBuilder {
    pageidentifier: Option<PageIdentifier>,
    page_size: Option<i64>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<i64>,
    total_records: Option<i64>,
}

impl BatchSummaryBuilder {
    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn total_net_amount(mut self, value: f64) -> Self {
        self.total_net_amount = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchSummary`].
    pub fn build(self) -> Result<BatchSummary, BuildError> {
        Ok(BatchSummary {
            pageidentifier: self.pageidentifier,
            page_size: self.page_size,
            total_amount: self.total_amount,
            total_net_amount: self.total_net_amount,
            total_pages: self.total_pages,
            total_records: self.total_records,
        })
    }
}
