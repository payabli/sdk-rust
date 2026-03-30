pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigureOrganizationRequestApplePay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade: Option<Cascade>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganizationId>,
}

impl ConfigureOrganizationRequestApplePay {
    pub fn builder() -> ConfigureOrganizationRequestApplePayBuilder {
        <ConfigureOrganizationRequestApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigureOrganizationRequestApplePayBuilder {
    cascade: Option<Cascade>,
    is_enabled: Option<IsEnabled>,
    org_id: Option<OrganizationId>,
}

impl ConfigureOrganizationRequestApplePayBuilder {
    pub fn cascade(mut self, value: Cascade) -> Self {
        self.cascade = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn org_id(mut self, value: OrganizationId) -> Self {
        self.org_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigureOrganizationRequestApplePay`].
    pub fn build(self) -> Result<ConfigureOrganizationRequestApplePay, BuildError> {
        Ok(ConfigureOrganizationRequestApplePay {
            cascade: self.cascade,
            is_enabled: self.is_enabled,
            org_id: self.org_id,
        })
    }
}
