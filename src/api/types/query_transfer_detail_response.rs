pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTransferDetailResponse {
    /// List of transfer detail records
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<TransferDetailRecord>,
    /// Summary of the transfer details query
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QueryTransferSummary,
}

impl QueryTransferDetailResponse {
    pub fn builder() -> QueryTransferDetailResponseBuilder {
        <QueryTransferDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTransferDetailResponseBuilder {
    records: Option<Vec<TransferDetailRecord>>,
    summary: Option<QueryTransferSummary>,
}

impl QueryTransferDetailResponseBuilder {
    pub fn records(mut self, value: Vec<TransferDetailRecord>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QueryTransferSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTransferDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](QueryTransferDetailResponseBuilder::records)
    /// - [`summary`](QueryTransferDetailResponseBuilder::summary)
    pub fn build(self) -> Result<QueryTransferDetailResponse, BuildError> {
        Ok(QueryTransferDetailResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
