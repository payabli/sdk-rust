pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferQueryResponse {
    #[serde(rename = "Records")]
    pub records: Vec<Transfer>,
    #[serde(rename = "Summary")]
    pub summary: TransferSummary,
}
