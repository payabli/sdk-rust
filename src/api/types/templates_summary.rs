pub use crate::prelude::*;

/// Pagination metadata for a template list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TemplatesSummary {
    /// The total number of pages available.
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: i64,
    /// The total number of templates across all pages.
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: i64,
    /// The number of templates per page.
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
}

impl TemplatesSummary {
    pub fn builder() -> TemplatesSummaryBuilder {
        <TemplatesSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatesSummaryBuilder {
    total_pages: Option<i64>,
    total_records: Option<i64>,
    page_size: Option<i64>,
}

impl TemplatesSummaryBuilder {
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

    /// Consumes the builder and constructs a [`TemplatesSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_pages`](TemplatesSummaryBuilder::total_pages)
    /// - [`total_records`](TemplatesSummaryBuilder::total_records)
    /// - [`page_size`](TemplatesSummaryBuilder::page_size)
    pub fn build(self) -> Result<TemplatesSummary, BuildError> {
        Ok(TemplatesSummary {
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
        })
    }
}
