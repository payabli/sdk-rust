pub use crate::prelude::*;

/// Query parameters for CustomerBasicStats
///
/// Request type for the CustomerBasicStatsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomerBasicStatsQueryRequest {
    /// List of parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}
