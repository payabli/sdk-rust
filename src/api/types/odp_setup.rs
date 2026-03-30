pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl OdpSetup {
    pub fn builder() -> OdpSetupBuilder {
        <OdpSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OdpSetupBuilder {
    allow_ach: Option<bool>,
    allow_checks: Option<bool>,
    allow_v_card: Option<bool>,
    processing_region: Option<OdpSetupProcessingRegion>,
    processor: Option<String>,
    issuer_network_settings_id: Option<String>,
}

impl OdpSetupBuilder {
    pub fn allow_ach(mut self, value: bool) -> Self {
        self.allow_ach = Some(value);
        self
    }

    pub fn allow_checks(mut self, value: bool) -> Self {
        self.allow_checks = Some(value);
        self
    }

    pub fn allow_v_card(mut self, value: bool) -> Self {
        self.allow_v_card = Some(value);
        self
    }

    pub fn processing_region(mut self, value: OdpSetupProcessingRegion) -> Self {
        self.processing_region = Some(value);
        self
    }

    pub fn processor(mut self, value: impl Into<String>) -> Self {
        self.processor = Some(value.into());
        self
    }

    pub fn issuer_network_settings_id(mut self, value: impl Into<String>) -> Self {
        self.issuer_network_settings_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OdpSetup`].
    pub fn build(self) -> Result<OdpSetup, BuildError> {
        Ok(OdpSetup {
            allow_ach: self.allow_ach,
            allow_checks: self.allow_checks,
            allow_v_card: self.allow_v_card,
            processing_region: self.processing_region,
            processor: self.processor,
            issuer_network_settings_id: self.issuer_network_settings_id,
        })
    }
}
