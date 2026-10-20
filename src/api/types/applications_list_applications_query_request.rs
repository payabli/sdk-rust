pub use crate::prelude::*;

/// Query parameters for ListApplications
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationsListApplicationsQueryRequest {
    /// The page number to return, starting at 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The number of applications per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ApplicationsListApplicationsQueryRequest {
    pub fn builder() -> ApplicationsListApplicationsQueryRequestBuilder {
        <ApplicationsListApplicationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationsListApplicationsQueryRequestBuilder {
    page: Option<i64>,
    limit: Option<i64>,
}

impl ApplicationsListApplicationsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationsListApplicationsQueryRequest`].
    pub fn build(self) -> Result<ApplicationsListApplicationsQueryRequest, BuildError> {
        Ok(ApplicationsListApplicationsQueryRequest {
            page: self.page,
            limit: self.limit,
        })
    }
}
