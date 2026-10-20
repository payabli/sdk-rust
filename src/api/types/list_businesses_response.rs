pub use crate::prelude::*;

/// A page of businesses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponse {
    /// The businesses on this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BusinessSummaryResponse>>,
    /// Pagination metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl ListBusinessesResponse {
    pub fn builder() -> ListBusinessesResponseBuilder {
        <ListBusinessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseBuilder {
    data: Option<Vec<BusinessSummaryResponse>>,
    pagination: Option<Pagination>,
}

impl ListBusinessesResponseBuilder {
    pub fn data(mut self, value: Vec<BusinessSummaryResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponse`].
    pub fn build(self) -> Result<ListBusinessesResponse, BuildError> {
        Ok(ListBusinessesResponse {
            data: self.data,
            pagination: self.pagination,
        })
    }
}
