pub use crate::prelude::*;

/// Query parameters for ListOrganizations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationsListOrganizationsQueryRequest {
    /// Filter by the business the organization belongs to.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
    /// Filter by organization status. One of `active`, `inactive`, `suspended`, or `closed`. Any other value returns `400`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Free-text search across organizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl OrganizationsListOrganizationsQueryRequest {
    pub fn builder() -> OrganizationsListOrganizationsQueryRequestBuilder {
        <OrganizationsListOrganizationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationsListOrganizationsQueryRequestBuilder {
    business_reference: Option<String>,
    status: Option<String>,
    search: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
}

impl OrganizationsListOrganizationsQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`OrganizationsListOrganizationsQueryRequest`].
    pub fn build(self) -> Result<OrganizationsListOrganizationsQueryRequest, BuildError> {
        Ok(OrganizationsListOrganizationsQueryRequest {
            business_reference: self.business_reference,
            status: self.status,
            search: self.search,
            limit: self.limit,
            page: self.page,
        })
    }
}
