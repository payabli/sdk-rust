pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentMethodDomainRequestWallet {
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl UpdatePaymentMethodDomainRequestWallet {
    pub fn builder() -> UpdatePaymentMethodDomainRequestWalletBuilder {
        <UpdatePaymentMethodDomainRequestWalletBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentMethodDomainRequestWalletBuilder {
    is_enabled: Option<IsEnabled>,
}

impl UpdatePaymentMethodDomainRequestWalletBuilder {
    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePaymentMethodDomainRequestWallet`].
    pub fn build(self) -> Result<UpdatePaymentMethodDomainRequestWallet, BuildError> {
        Ok(UpdatePaymentMethodDomainRequestWallet {
            is_enabled: self.is_enabled,
        })
    }
}
