pub use crate::prelude::*;

/// Response body for queries about virtual card transactions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardTransactionQueryResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: VCardSummary,
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<VCardTransactionRecord>,
}

impl VCardTransactionQueryResponse {
    pub fn builder() -> VCardTransactionQueryResponseBuilder {
        <VCardTransactionQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardTransactionQueryResponseBuilder {
    summary: Option<VCardSummary>,
    records: Option<Vec<VCardTransactionRecord>>,
}

impl VCardTransactionQueryResponseBuilder {
    pub fn summary(mut self, value: VCardSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<VCardTransactionRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VCardTransactionQueryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](VCardTransactionQueryResponseBuilder::summary)
    /// - [`records`](VCardTransactionQueryResponseBuilder::records)
    pub fn build(self) -> Result<VCardTransactionQueryResponse, BuildError> {
        Ok(VCardTransactionQueryResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
