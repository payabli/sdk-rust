pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrganizationUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade: Option<WalletCascade>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<WalletIsEnabled>,
}
