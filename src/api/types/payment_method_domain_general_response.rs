pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDomainGeneralResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PaymentMethodDomainApiResponse>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: String,
}

impl PaymentMethodDomainGeneralResponse {
    pub fn builder() -> PaymentMethodDomainGeneralResponseBuilder {
        <PaymentMethodDomainGeneralResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDomainGeneralResponseBuilder {
    is_success: Option<IsSuccess>,
    pageidentifier: Option<PageIdentifier>,
    response_data: Option<PaymentMethodDomainApiResponse>,
    response_text: Option<String>,
}

impl PaymentMethodDomainGeneralResponseBuilder {
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

    /// Consumes the builder and constructs a [`PaymentMethodDomainGeneralResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PaymentMethodDomainGeneralResponseBuilder::response_text)
    pub fn build(self) -> Result<PaymentMethodDomainGeneralResponse, BuildError> {
        Ok(PaymentMethodDomainGeneralResponse {
            is_success: self.is_success,
            pageidentifier: self.pageidentifier,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
