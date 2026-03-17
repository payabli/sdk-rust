pub use crate::prelude::*;

/// Response payload for queries related to vendors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseVendors {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<VendorQueryRecord>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
