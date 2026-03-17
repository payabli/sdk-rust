pub use crate::prelude::*;

/// Response payload for queries related to transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseTransactions {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<TransactionQueryRecords>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
