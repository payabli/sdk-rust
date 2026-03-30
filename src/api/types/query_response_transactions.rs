pub use crate::prelude::*;

/// Response payload for queries related to transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseTransactions {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<TransactionQueryRecords>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}

impl QueryResponseTransactions {
    pub fn builder() -> QueryResponseTransactionsBuilder {
        <QueryResponseTransactionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseTransactionsBuilder {
    records: Option<Vec<TransactionQueryRecords>>,
    summary: Option<QuerySummary>,
}

impl QueryResponseTransactionsBuilder {
    pub fn records(mut self, value: Vec<TransactionQueryRecords>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseTransactions`].
    pub fn build(self) -> Result<QueryResponseTransactions, BuildError> {
        Ok(QueryResponseTransactions {
            records: self.records,
            summary: self.summary,
        })
    }
}
