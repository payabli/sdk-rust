pub use crate::prelude::*;

/// Settings for wallet payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodElementSettings {
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<MethodElementSettingsApplePay>,
}
