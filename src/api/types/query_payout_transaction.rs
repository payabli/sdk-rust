pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPayoutTransaction {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryPayoutTransactionRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QueryPayoutTransactionSummary>,
}

impl QueryPayoutTransaction {
    pub fn builder() -> QueryPayoutTransactionBuilder {
        <QueryPayoutTransactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPayoutTransactionBuilder {
    records: Option<Vec<QueryPayoutTransactionRecordsItem>>,
    summary: Option<QueryPayoutTransactionSummary>,
}

impl QueryPayoutTransactionBuilder {
    pub fn records(mut self, value: Vec<QueryPayoutTransactionRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QueryPayoutTransactionSummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryPayoutTransaction`].
    pub fn build(self) -> Result<QueryPayoutTransaction, BuildError> {
        Ok(QueryPayoutTransaction {
            records: self.records,
            summary: self.summary,
        })
    }
}
