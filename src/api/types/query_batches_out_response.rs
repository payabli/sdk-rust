pub use crate::prelude::*;

/// Response body for queries about money out batches.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryBatchesOutResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<QueryBatchesOutResponseRecordsItem>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: BatchSummary,
}

impl QueryBatchesOutResponse {
    pub fn builder() -> QueryBatchesOutResponseBuilder {
        <QueryBatchesOutResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBatchesOutResponseBuilder {
    records: Option<Vec<QueryBatchesOutResponseRecordsItem>>,
    summary: Option<BatchSummary>,
}

impl QueryBatchesOutResponseBuilder {
    pub fn records(mut self, value: Vec<QueryBatchesOutResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: BatchSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryBatchesOutResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](QueryBatchesOutResponseBuilder::records)
    /// - [`summary`](QueryBatchesOutResponseBuilder::summary)
    pub fn build(self) -> Result<QueryBatchesOutResponse, BuildError> {
        Ok(QueryBatchesOutResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
