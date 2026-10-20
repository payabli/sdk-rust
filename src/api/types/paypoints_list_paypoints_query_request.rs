pub use crate::prelude::*;

/// Query parameters for ListPaypoints
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointsListPaypointsQueryRequest {
    /// Filter by the business the paypoint belongs to.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
    /// Filter by paypoint status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Free-text search across paypoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl PaypointsListPaypointsQueryRequest {
    pub fn builder() -> PaypointsListPaypointsQueryRequestBuilder {
        <PaypointsListPaypointsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointsListPaypointsQueryRequestBuilder {
    business_reference: Option<String>,
    status: Option<String>,
    search: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
}

impl PaypointsListPaypointsQueryRequestBuilder {
    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointsListPaypointsQueryRequest`].
    pub fn build(self) -> Result<PaypointsListPaypointsQueryRequest, BuildError> {
        Ok(PaypointsListPaypointsQueryRequest {
            business_reference: self.business_reference,
            status: self.status,
            search: self.search,
            limit: self.limit,
            page: self.page,
        })
    }
}
