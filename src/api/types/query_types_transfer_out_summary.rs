pub use crate::prelude::*;

/// Summary information for outbound transfer queries.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutSummary {
    /// Number of pages in the response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Number of records in the response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
    /// Number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl TransferOutSummary {
    pub fn builder() -> TransferOutSummaryBuilder {
        <TransferOutSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutSummaryBuilder {
    total_pages: Option<i64>,
    total_records: Option<i64>,
    page_size: Option<i64>,
}

impl TransferOutSummaryBuilder {
    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutSummary`].
    pub fn build(self) -> Result<TransferOutSummary, BuildError> {
        Ok(TransferOutSummary {
            total_pages: self.total_pages,
            total_records: self.total_records,
            page_size: self.page_size,
        })
    }
}
