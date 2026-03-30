pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTransferResponse {
    /// Summary information about the transfers.
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QueryTransferSummary,
    /// List of transfer transaction records.
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<TransactionQueryRecords>,
}

impl QueryTransferResponse {
    pub fn builder() -> QueryTransferResponseBuilder {
        <QueryTransferResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTransferResponseBuilder {
    summary: Option<QueryTransferSummary>,
    records: Option<Vec<TransactionQueryRecords>>,
}

impl QueryTransferResponseBuilder {
    pub fn summary(mut self, value: QueryTransferSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<TransactionQueryRecords>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTransferResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](QueryTransferResponseBuilder::summary)
    /// - [`records`](QueryTransferResponseBuilder::records)
    pub fn build(self) -> Result<QueryTransferResponse, BuildError> {
        Ok(QueryTransferResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
