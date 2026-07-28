pub use crate::prelude::*;

/// Pagination and totals for a case list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaseListSummary {
    /// The total number of matching cases.
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
    /// Not used for cases; returned as part of the shared list envelope.
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_amount: f64,
    /// Not used for cases; returned as part of the shared list envelope.
    #[serde(rename = "totalNetAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_net_amount: f64,
    /// The total number of pages.
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    /// The number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    /// An opaque page identifier, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<String>,
}

impl CaseListSummary {
    pub fn builder() -> CaseListSummaryBuilder {
        <CaseListSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaseListSummaryBuilder {
    total_records: Option<i64>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<i64>,
    page_size: Option<i64>,
    pageidentifier: Option<String>,
}

impl CaseListSummaryBuilder {
    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
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

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: impl Into<String>) -> Self {
        self.pageidentifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CaseListSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_records`](CaseListSummaryBuilder::total_records)
    /// - [`total_amount`](CaseListSummaryBuilder::total_amount)
    /// - [`total_net_amount`](CaseListSummaryBuilder::total_net_amount)
    /// - [`total_pages`](CaseListSummaryBuilder::total_pages)
    /// - [`page_size`](CaseListSummaryBuilder::page_size)
    pub fn build(self) -> Result<CaseListSummary, BuildError> {
        Ok(CaseListSummary {
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            total_net_amount: self
                .total_net_amount
                .ok_or_else(|| BuildError::missing_field("total_net_amount"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            pageidentifier: self.pageidentifier,
        })
    }
}
