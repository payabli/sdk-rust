pub use crate::prelude::*;

/// Query parameters for VendorBasicStats
///
/// Request type for the VendorBasicStatsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorBasicStatsQueryRequest {
    /// List of parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}
