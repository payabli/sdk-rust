pub use crate::prelude::*;

/// Pagination metadata for a list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Pagination {
    /// The current page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The total number of pages available.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// The total number of records across all pages.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
    /// The number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Whether a next page is available.
    #[serde(rename = "hasNext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// Whether a previous page is available.
    #[serde(rename = "hasPrevious")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_previous: Option<bool>,
}

impl Pagination {
    pub fn builder() -> PaginationBuilder {
        <PaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationBuilder {
    page: Option<i64>,
    total_pages: Option<i64>,
    total_records: Option<i64>,
    page_size: Option<i64>,
    has_next: Option<bool>,
    has_previous: Option<bool>,
}

impl PaginationBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
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

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn has_next(mut self, value: bool) -> Self {
        self.has_next = Some(value);
        self
    }

    pub fn has_previous(mut self, value: bool) -> Self {
        self.has_previous = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pagination`].
    pub fn build(self) -> Result<Pagination, BuildError> {
        Ok(Pagination {
            page: self.page,
            total_pages: self.total_pages,
            total_records: self.total_records,
            page_size: self.page_size,
            has_next: self.has_next,
            has_previous: self.has_previous,
        })
    }
}
