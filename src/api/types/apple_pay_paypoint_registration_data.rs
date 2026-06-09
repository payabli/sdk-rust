pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplePayPaypointRegistrationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Entry>,
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    /// The wallet type. In this context it will always be `applepay`.
    #[serde(rename = "walletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    #[serde(rename = "walletData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_data: Option<AppleWalletData>,
}

impl ApplePayPaypointRegistrationData {
    pub fn builder() -> ApplePayPaypointRegistrationDataBuilder {
        <ApplePayPaypointRegistrationDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplePayPaypointRegistrationDataBuilder {
    entry: Option<Entry>,
    is_enabled: Option<IsEnabled>,
    wallet_type: Option<String>,
    wallet_data: Option<AppleWalletData>,
}

impl ApplePayPaypointRegistrationDataBuilder {
    pub fn entry(mut self, value: Entry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }

    pub fn wallet_data(mut self, value: AppleWalletData) -> Self {
        self.wallet_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplePayPaypointRegistrationData`].
    pub fn build(self) -> Result<ApplePayPaypointRegistrationData, BuildError> {
        Ok(ApplePayPaypointRegistrationData {
            entry: self.entry,
            is_enabled: self.is_enabled,
            wallet_type: self.wallet_type,
            wallet_data: self.wallet_data,
        })
    }
}
