pub use crate::prelude::*;

/// Response body for queries about batches.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBatchesResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<QueryBatchesResponseRecordsItem>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: BatchSummary,
}

impl QueryBatchesResponse {
    pub fn builder() -> QueryBatchesResponseBuilder {
        <QueryBatchesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesResponseBuilder {
    records: Option<Vec<QueryBatchesResponseRecordsItem>>,
    summary: Option<BatchSummary>,
}

impl QueryBatchesResponseBuilder {
    pub fn records(mut self, value: Vec<QueryBatchesResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: BatchSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](QueryBatchesResponseBuilder::records)
    /// - [`summary`](QueryBatchesResponseBuilder::summary)
    pub fn build(self) -> Result<QueryBatchesResponse, BuildError> {
        Ok(QueryBatchesResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
