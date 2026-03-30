pub use crate::prelude::*;

/// Response body for queries about outbound transfers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutQueryResponse {
    /// Summary information about the transfers.
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: TransferOutSummary,
    /// List of outbound transfer records.
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<TransferOutRecord>,
}

impl TransferOutQueryResponse {
    pub fn builder() -> TransferOutQueryResponseBuilder {
        <TransferOutQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutQueryResponseBuilder {
    summary: Option<TransferOutSummary>,
    records: Option<Vec<TransferOutRecord>>,
}

impl TransferOutQueryResponseBuilder {
    pub fn summary(mut self, value: TransferOutSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<TransferOutRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutQueryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](TransferOutQueryResponseBuilder::summary)
    /// - [`records`](TransferOutQueryResponseBuilder::records)
    pub fn build(self) -> Result<TransferOutQueryResponse, BuildError> {
        Ok(TransferOutQueryResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
