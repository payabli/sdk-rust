pub use crate::prelude::*;

/// Response body for queries about batch details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBatchesDetailResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<BatchDetailResponseRecord>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: BatchDetailResponseSummary,
}

impl QueryBatchesDetailResponse {
    pub fn builder() -> QueryBatchesDetailResponseBuilder {
        <QueryBatchesDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesDetailResponseBuilder {
    records: Option<Vec<BatchDetailResponseRecord>>,
    summary: Option<BatchDetailResponseSummary>,
}

impl QueryBatchesDetailResponseBuilder {
    pub fn records(mut self, value: Vec<BatchDetailResponseRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: BatchDetailResponseSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](QueryBatchesDetailResponseBuilder::records)
    /// - [`summary`](QueryBatchesDetailResponseBuilder::summary)
    pub fn build(self) -> Result<QueryBatchesDetailResponse, BuildError> {
        Ok(QueryBatchesDetailResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
