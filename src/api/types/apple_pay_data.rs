pub use crate::prelude::*;

/// Details about the status of the Apple Pay service.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplePayData {
    /// This object is only returned when the domain verification check
    /// fails. If a domain has failed validation, this object contains
    /// information about the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ApplePayStatusData>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl ApplePayData {
    pub fn builder() -> ApplePayDataBuilder {
        <ApplePayDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplePayDataBuilder {
    data: Option<ApplePayStatusData>,
    is_enabled: Option<IsEnabled>,
}

impl ApplePayDataBuilder {
    pub fn data(mut self, value: ApplePayStatusData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplePayData`].
    pub fn build(self) -> Result<ApplePayData, BuildError> {
        Ok(ApplePayData {
            data: self.data,
            is_enabled: self.is_enabled,
        })
    }
}
