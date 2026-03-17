pub use crate::prelude::*;

/// Details about the status of the Apple Pay service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplePayData {
    /// This object is only returned when the domain verification check fails. If a domain has failed validation, this object contains information about the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ApplePayStatusData>,
    /// When `true`, Apple Pay is enabled.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}
