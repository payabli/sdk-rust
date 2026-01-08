pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigurePaypointRequestApplePay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Entry>,
    /// When `true`, Apple Pay is enabled.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}
