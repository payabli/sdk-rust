pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferQueryResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<Transfer>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: TransferSummary,
}

impl TransferQueryResponse {
    pub fn builder() -> TransferQueryResponseBuilder {
        <TransferQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferQueryResponseBuilder {
    records: Option<Vec<Transfer>>,
    summary: Option<TransferSummary>,
}

impl TransferQueryResponseBuilder {
    pub fn records(mut self, value: Vec<Transfer>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: TransferSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferQueryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](TransferQueryResponseBuilder::records)
    /// - [`summary`](TransferQueryResponseBuilder::summary)
    pub fn build(self) -> Result<TransferQueryResponse, BuildError> {
        Ok(TransferQueryResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
