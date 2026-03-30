pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade: Option<WalletCascade>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<WalletIsEnabled>,
}

impl OrganizationUpdates {
    pub fn builder() -> OrganizationUpdatesBuilder {
        <OrganizationUpdatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationUpdatesBuilder {
    cascade: Option<WalletCascade>,
    is_enabled: Option<WalletIsEnabled>,
}

impl OrganizationUpdatesBuilder {
    pub fn cascade(mut self, value: WalletCascade) -> Self {
        self.cascade = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: WalletIsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationUpdates`].
    pub fn build(self) -> Result<OrganizationUpdates, BuildError> {
        Ok(OrganizationUpdates {
            cascade: self.cascade,
            is_enabled: self.is_enabled,
        })
    }
}
