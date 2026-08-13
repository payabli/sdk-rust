pub use crate::prelude::*;

/// A page of billing profiles that belong to an organization, returned by the
/// List profiles endpoint. This is the data behind the Profile Library table in
/// the Payabli Portal.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingProfileQueryResponse {
    #[serde(default)]
    pub summary: BillingProfileSummary,
    /// The billing profiles on this page. Empty when the org has no profiles.
    #[serde(default)]
    pub records: Vec<BillingProfileRecord>,
}

impl BillingProfileQueryResponse {
    pub fn builder() -> BillingProfileQueryResponseBuilder {
        <BillingProfileQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingProfileQueryResponseBuilder {
    summary: Option<BillingProfileSummary>,
    records: Option<Vec<BillingProfileRecord>>,
}

impl BillingProfileQueryResponseBuilder {
    pub fn summary(mut self, value: BillingProfileSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<BillingProfileRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingProfileQueryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](BillingProfileQueryResponseBuilder::summary)
    /// - [`records`](BillingProfileQueryResponseBuilder::records)
    pub fn build(self) -> Result<BillingProfileQueryResponse, BuildError> {
        Ok(BillingProfileQueryResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
