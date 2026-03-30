pub use crate::prelude::*;

/// Google Pay configuration information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainRequestGooglePay {
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl AddPaymentMethodDomainRequestGooglePay {
    pub fn builder() -> AddPaymentMethodDomainRequestGooglePayBuilder {
        <AddPaymentMethodDomainRequestGooglePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPaymentMethodDomainRequestGooglePayBuilder {
    is_enabled: Option<IsEnabled>,
}

impl AddPaymentMethodDomainRequestGooglePayBuilder {
    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPaymentMethodDomainRequestGooglePay`].
    pub fn build(self) -> Result<AddPaymentMethodDomainRequestGooglePay, BuildError> {
        Ok(AddPaymentMethodDomainRequestGooglePay {
            is_enabled: self.is_enabled,
        })
    }
}
