pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigurePaypointRequestGooglePay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Entry>,
    /// When `true`, Google Pay is enabled.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl ConfigurePaypointRequestGooglePay {
    pub fn builder() -> ConfigurePaypointRequestGooglePayBuilder {
        <ConfigurePaypointRequestGooglePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigurePaypointRequestGooglePayBuilder {
    entry: Option<Entry>,
    is_enabled: Option<IsEnabled>,
}

impl ConfigurePaypointRequestGooglePayBuilder {
    pub fn entry(mut self, value: Entry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigurePaypointRequestGooglePay`].
    pub fn build(self) -> Result<ConfigurePaypointRequestGooglePay, BuildError> {
        Ok(ConfigurePaypointRequestGooglePay {
            entry: self.entry,
            is_enabled: self.is_enabled,
        })
    }
}
