pub use crate::prelude::*;

/// The wallet data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GoogleWalletData {
    /// The Google Pay merchant identifier.
    #[serde(rename = "gatewayMerchantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_merchant_id: Option<String>,
    /// The Google Pay gateway identifier.
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
}

impl GoogleWalletData {
    pub fn builder() -> GoogleWalletDataBuilder {
        <GoogleWalletDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoogleWalletDataBuilder {
    gateway_merchant_id: Option<String>,
    gateway_id: Option<String>,
}

impl GoogleWalletDataBuilder {
    pub fn gateway_merchant_id(mut self, value: impl Into<String>) -> Self {
        self.gateway_merchant_id = Some(value.into());
        self
    }

    pub fn gateway_id(mut self, value: impl Into<String>) -> Self {
        self.gateway_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GoogleWalletData`].
    pub fn build(self) -> Result<GoogleWalletData, BuildError> {
        Ok(GoogleWalletData {
            gateway_merchant_id: self.gateway_merchant_id,
            gateway_id: self.gateway_id,
        })
    }
}
