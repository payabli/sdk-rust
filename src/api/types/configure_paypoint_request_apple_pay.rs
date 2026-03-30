pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigurePaypointRequestApplePay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Entry>,
    /// When `true`, Apple Pay is enabled.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl ConfigurePaypointRequestApplePay {
    pub fn builder() -> ConfigurePaypointRequestApplePayBuilder {
        <ConfigurePaypointRequestApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigurePaypointRequestApplePayBuilder {
    entry: Option<Entry>,
    is_enabled: Option<IsEnabled>,
}

impl ConfigurePaypointRequestApplePayBuilder {
    pub fn entry(mut self, value: Entry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigurePaypointRequestApplePay`].
    pub fn build(self) -> Result<ConfigurePaypointRequestApplePay, BuildError> {
        Ok(ConfigurePaypointRequestApplePay {
            entry: self.entry,
            is_enabled: self.is_enabled,
        })
    }
}
