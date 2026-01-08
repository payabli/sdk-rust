pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryResponseNotificationReportsRecordsItem {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Unique identifier for the report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Indicator of whether the report can be downloaded.
    #[serde(rename = "isDownloadable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downloadable: Option<bool>,
    /// Name of the report.
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}
