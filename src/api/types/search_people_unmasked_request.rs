pub use crate::prelude::*;

/// Request for SearchPeopleUnmasked (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchPeopleUnmaskedRequest {
    /// The entry name of your organization, from `OrgEntryName` on the organization. With a paypoint's entry name instead, the search covers only that paypoint's business.
    #[serde(skip)]
    #[serde(default)]
    pub entry: String,
    /// The ID of your organization.
    #[serde(rename = "orgId")]
    #[serde(skip)]
    #[serde(default)]
    pub org_id: i64,
    pub body: PeopleSearchRequest,
}

impl SearchPeopleUnmaskedRequest {
    pub fn builder() -> SearchPeopleUnmaskedRequestBuilder {
        <SearchPeopleUnmaskedRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchPeopleUnmaskedRequestBuilder {
    entry: Option<String>,
    org_id: Option<i64>,
    body: Option<PeopleSearchRequest>,
}

impl SearchPeopleUnmaskedRequestBuilder {
    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn body(mut self, value: PeopleSearchRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchPeopleUnmaskedRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry`](SearchPeopleUnmaskedRequestBuilder::entry)
    /// - [`org_id`](SearchPeopleUnmaskedRequestBuilder::org_id)
    /// - [`body`](SearchPeopleUnmaskedRequestBuilder::body)
    pub fn build(self) -> Result<SearchPeopleUnmaskedRequest, BuildError> {
        Ok(SearchPeopleUnmaskedRequest {
            entry: self
                .entry
                .ok_or_else(|| BuildError::missing_field("entry"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
