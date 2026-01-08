pub use crate::prelude::*;

/// Query parameters for SubStats
///
/// Request type for the SubStatsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubStatsQueryRequest {
    /// List of parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}
