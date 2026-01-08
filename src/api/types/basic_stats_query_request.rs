pub use crate::prelude::*;

/// Query parameters for BasicStats
///
/// Request type for the BasicStatsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BasicStatsQueryRequest {
    /// Used with `custom` mode. The end date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// - mm-dd-YYYY
    /// - mm/dd/YYYY
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// List of parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// Used with `custom` mode. The start date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// -  mm-dd-YYYY
    /// - mm/dd/YYYY
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}
