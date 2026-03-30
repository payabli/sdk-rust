pub use crate::prelude::*;

/// Response for the add payment method domain operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(default)]
    pub pageidentifier: PageIdentifier,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: PaymentMethodDomainApiResponse,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: String,
}

impl AddPaymentMethodDomainApiResponse {
    pub fn builder() -> AddPaymentMethodDomainApiResponseBuilder {
        <AddPaymentMethodDomainApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPaymentMethodDomainApiResponseBuilder {
    is_success: Option<IsSuccess>,
    pageidentifier: Option<PageIdentifier>,
    response_data: Option<PaymentMethodDomainApiResponse>,
    response_text: Option<String>,
}

impl AddPaymentMethodDomainApiResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: PaymentMethodDomainApiResponse) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: impl Into<String>) -> Self {
        self.response_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddPaymentMethodDomainApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](AddPaymentMethodDomainApiResponseBuilder::is_success)
    /// - [`pageidentifier`](AddPaymentMethodDomainApiResponseBuilder::pageidentifier)
    /// - [`response_data`](AddPaymentMethodDomainApiResponseBuilder::response_data)
    /// - [`response_text`](AddPaymentMethodDomainApiResponseBuilder::response_text)
    pub fn build(self) -> Result<AddPaymentMethodDomainApiResponse, BuildError> {
        Ok(AddPaymentMethodDomainApiResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            pageidentifier: self
                .pageidentifier
                .ok_or_else(|| BuildError::missing_field("pageidentifier"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
