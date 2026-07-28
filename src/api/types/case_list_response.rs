pub use crate::prelude::*;

/// A paginated list of cases.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaseListResponse {
    #[serde(default)]
    pub summary: CaseListSummary,
    /// The cases on this page. Each record is a full case object.
    #[serde(default)]
    pub records: Vec<CaseResponse>,
}

impl CaseListResponse {
    pub fn builder() -> CaseListResponseBuilder {
        <CaseListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaseListResponseBuilder {
    summary: Option<CaseListSummary>,
    records: Option<Vec<CaseResponse>>,
}

impl CaseListResponseBuilder {
    pub fn summary(mut self, value: CaseListSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<CaseResponse>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaseListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](CaseListResponseBuilder::summary)
    /// - [`records`](CaseListResponseBuilder::records)
    pub fn build(self) -> Result<CaseListResponse, BuildError> {
        Ok(CaseListResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
