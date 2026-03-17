pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferSummary {
    #[serde(rename = "totalPages")]
    pub total_pages: Totalpages,
    #[serde(rename = "totalRecords")]
    pub total_records: Totalrecords,
    #[serde(rename = "pageSize")]
    pub page_size: Pagesize,
}
