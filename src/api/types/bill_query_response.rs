pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillQueryResponse {
    /// Summary statistics for the bill query response.
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<BillQueryResponseSummary>,
    /// Array of bill records returned by the query.
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<BillQueryRecord2>>,
}
