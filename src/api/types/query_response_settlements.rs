pub use crate::prelude::*;

/// Describes the response for settlement queries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseSettlements {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseSettlementsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QueryResponseSettlementsSummary>,
}
