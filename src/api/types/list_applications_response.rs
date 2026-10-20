pub use crate::prelude::*;

/// A page of applications.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApplicationsResponse {
    /// The applications on this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ApplicationListItem>>,
    /// The total number of applications across all pages.
    #[serde(default)]
    pub total: i64,
    /// The current page number.
    #[serde(default)]
    pub page: i64,
    /// The number of applications per page.
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
}

impl ListApplicationsResponse {
    pub fn builder() -> ListApplicationsResponseBuilder {
        <ListApplicationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApplicationsResponseBuilder {
    data: Option<Vec<ApplicationListItem>>,
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListApplicationsResponseBuilder {
    pub fn data(mut self, value: Vec<ApplicationListItem>) -> Self {
        self.data = Some(value);
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

    /// Consumes the builder and constructs a [`ListApplicationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](ListApplicationsResponseBuilder::total)
    /// - [`page`](ListApplicationsResponseBuilder::page)
    /// - [`page_size`](ListApplicationsResponseBuilder::page_size)
    pub fn build(self) -> Result<ListApplicationsResponse, BuildError> {
        Ok(ListApplicationsResponse {
            data: self.data,
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
