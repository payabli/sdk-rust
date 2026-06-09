pub use crate::prelude::*;

/// Response body for queries about outbound transfer details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutDetailQueryResponse {
    /// Summary information about the transfer details.
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QueryTransferSummary,
    /// List of outbound transfer detail records.
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<TransferOutDetailRecord>,
}

impl TransferOutDetailQueryResponse {
    pub fn builder() -> TransferOutDetailQueryResponseBuilder {
        <TransferOutDetailQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailQueryResponseBuilder {
    summary: Option<QueryTransferSummary>,
    records: Option<Vec<TransferOutDetailRecord>>,
}

impl TransferOutDetailQueryResponseBuilder {
    pub fn summary(mut self, value: QueryTransferSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<TransferOutDetailRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailQueryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](TransferOutDetailQueryResponseBuilder::summary)
    /// - [`records`](TransferOutDetailQueryResponseBuilder::records)
    pub fn build(self) -> Result<TransferOutDetailQueryResponse, BuildError> {
        Ok(TransferOutDetailQueryResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
