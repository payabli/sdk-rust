pub use crate::prelude::*;

/// Apple Pay configuration information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainRequestApplePay {
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl AddPaymentMethodDomainRequestApplePay {
    pub fn builder() -> AddPaymentMethodDomainRequestApplePayBuilder {
        <AddPaymentMethodDomainRequestApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPaymentMethodDomainRequestApplePayBuilder {
    is_enabled: Option<IsEnabled>,
}

impl AddPaymentMethodDomainRequestApplePayBuilder {
    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPaymentMethodDomainRequestApplePay`].
    pub fn build(self) -> Result<AddPaymentMethodDomainRequestApplePay, BuildError> {
        Ok(AddPaymentMethodDomainRequestApplePay {
            is_enabled: self.is_enabled,
        })
    }
}
