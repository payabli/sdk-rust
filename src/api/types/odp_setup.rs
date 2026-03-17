pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OdpSetup {
    /// Enables or disables ACH payout functionality
    #[serde(rename = "allowAch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ach: Option<bool>,
    /// Enables or disables check printing payout functionality
    #[serde(rename = "allowChecks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_checks: Option<bool>,
    /// Enables or disables vCard payout functionality
    #[serde(rename = "allowVCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_v_card: Option<bool>,
    /// Region where payment processing occurs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_region: Option<OdpSetupProcessingRegion>,
    /// Payment processor identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
    /// Reference ID for the program enabled for ODP issuance
    #[serde(rename = "issuerNetworkSettingsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_network_settings_id: Option<String>,
}
