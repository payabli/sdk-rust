pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrganizationsResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<OrganizationQueryRecord>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QuerySummary,
}

impl ListOrganizationsResponse {
    pub fn builder() -> ListOrganizationsResponseBuilder {
        <ListOrganizationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrganizationsResponseBuilder {
    records: Option<Vec<OrganizationQueryRecord>>,
    summary: Option<QuerySummary>,
}

impl ListOrganizationsResponseBuilder {
    pub fn records(mut self, value: Vec<OrganizationQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrganizationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](ListOrganizationsResponseBuilder::records)
    /// - [`summary`](ListOrganizationsResponseBuilder::summary)
    pub fn build(self) -> Result<ListOrganizationsResponse, BuildError> {
        Ok(ListOrganizationsResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
