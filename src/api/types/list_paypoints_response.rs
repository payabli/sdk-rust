pub use crate::prelude::*;

/// A page of paypoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaypointsResponse {
    /// The paypoints on this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PaypointSummaryResponse>>,
    /// The total number of paypoints across all pages.
    #[serde(default)]
    pub total: i64,
    /// The current page number.
    #[serde(default)]
    pub page: i64,
    /// The number of paypoints per page.
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
}

impl ListPaypointsResponse {
    pub fn builder() -> ListPaypointsResponseBuilder {
        <ListPaypointsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaypointsResponseBuilder {
    items: Option<Vec<PaypointSummaryResponse>>,
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListPaypointsResponseBuilder {
    pub fn items(mut self, value: Vec<PaypointSummaryResponse>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaypointsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](ListPaypointsResponseBuilder::total)
    /// - [`page`](ListPaypointsResponseBuilder::page)
    /// - [`page_size`](ListPaypointsResponseBuilder::page_size)
    pub fn build(self) -> Result<ListPaypointsResponse, BuildError> {
        Ok(ListPaypointsResponse {
            items: self.items,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
        })
    }
}
