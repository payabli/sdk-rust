pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentMethodDomainRequest {
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
    #[serde(rename = "googlePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
}

impl UpdatePaymentMethodDomainRequest {
    pub fn builder() -> UpdatePaymentMethodDomainRequestBuilder {
        <UpdatePaymentMethodDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentMethodDomainRequestBuilder {
    apple_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
    google_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
}

impl UpdatePaymentMethodDomainRequestBuilder {
    pub fn apple_pay(mut self, value: UpdatePaymentMethodDomainRequestWallet) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn google_pay(mut self, value: UpdatePaymentMethodDomainRequestWallet) -> Self {
        self.google_pay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePaymentMethodDomainRequest`].
    pub fn build(self) -> Result<UpdatePaymentMethodDomainRequest, BuildError> {
        Ok(UpdatePaymentMethodDomainRequest {
            apple_pay: self.apple_pay,
            google_pay: self.google_pay,
        })
    }
}
