pub use crate::prelude::*;

/// Query parameters for ListDevice
///
/// Request type for the ListDeviceQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeviceQueryRequest {
    /// When `true`, the request retrieves an updated list of devices from the processor instead of returning a cached list of devices.
    #[serde(rename = "forceRefresh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_refresh: Option<bool>,
}
