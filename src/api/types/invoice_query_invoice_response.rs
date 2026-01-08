pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryInvoiceResponse {
    #[serde(rename = "Records")]
    pub records: Vec<QueryInvoiceResponseRecordsItem>,
    #[serde(rename = "Summary")]
    pub summary: QuerySummary,
}
