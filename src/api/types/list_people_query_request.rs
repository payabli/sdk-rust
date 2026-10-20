pub use crate::prelude::*;

/// Query parameters for ListPeople
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleQueryRequest {
    /// Filter by name (partial match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by email (partial match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Comma-separated verification status filter.
    #[serde(rename = "verificationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Field to sort by.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ListPeopleQueryRequest {
    pub fn builder() -> ListPeopleQueryRequestBuilder {
        <ListPeopleQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleQueryRequestBuilder {
    name: Option<String>,
    email: Option<String>,
    verification_status: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl ListPeopleQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn verification_status(mut self, value: impl Into<String>) -> Self {
        self.verification_status = Some(value.into());
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

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleQueryRequest`].
    pub fn build(self) -> Result<ListPeopleQueryRequest, BuildError> {
        Ok(ListPeopleQueryRequest {
            name: self.name,
            email: self.email,
            verification_status: self.verification_status,
            limit: self.limit,
            page: self.page,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        })
    }
}
