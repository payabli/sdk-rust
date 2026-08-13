pub use crate::prelude::*;

/// Pagination summary for the profile list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingProfileSummary {
    /// Opaque identifier for the returned page.
    #[serde(rename = "pageIdentifier")]
    #[serde(default)]
    pub page_identifier: String,
    /// Maximum number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    /// Total number of pages available.
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    /// Total number of profiles matching the query.
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
}

impl BillingProfileSummary {
    pub fn builder() -> BillingProfileSummaryBuilder {
        <BillingProfileSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingProfileSummaryBuilder {
    page_identifier: Option<String>,
    page_size: Option<i64>,
    total_pages: Option<i64>,
    total_records: Option<i64>,
}

impl BillingProfileSummaryBuilder {
    pub fn page_identifier(mut self, value: impl Into<String>) -> Self {
        self.page_identifier = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
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

    /// Consumes the builder and constructs a [`BillingProfileSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`page_identifier`](BillingProfileSummaryBuilder::page_identifier)
    /// - [`page_size`](BillingProfileSummaryBuilder::page_size)
    /// - [`total_pages`](BillingProfileSummaryBuilder::total_pages)
    /// - [`total_records`](BillingProfileSummaryBuilder::total_records)
    pub fn build(self) -> Result<BillingProfileSummary, BuildError> {
        Ok(BillingProfileSummary {
            page_identifier: self
                .page_identifier
                .ok_or_else(|| BuildError::missing_field("page_identifier"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
        })
    }
}
