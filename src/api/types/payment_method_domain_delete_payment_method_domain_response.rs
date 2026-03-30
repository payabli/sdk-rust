pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeletePaymentMethodDomainResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(default)]
    pub page_identifier: PageIdentifier,
    /// The deleted domain's domain ID.
    #[serde(rename = "responseData")]
    pub response_data: Responsedatanonobject,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl DeletePaymentMethodDomainResponse {
    pub fn builder() -> DeletePaymentMethodDomainResponseBuilder {
        <DeletePaymentMethodDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePaymentMethodDomainResponseBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_data: Option<Responsedatanonobject>,
    response_text: Option<ResponseText>,
}

impl DeletePaymentMethodDomainResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: Responsedatanonobject) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeletePaymentMethodDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](DeletePaymentMethodDomainResponseBuilder::is_success)
    /// - [`page_identifier`](DeletePaymentMethodDomainResponseBuilder::page_identifier)
    /// - [`response_data`](DeletePaymentMethodDomainResponseBuilder::response_data)
    /// - [`response_text`](DeletePaymentMethodDomainResponseBuilder::response_text)
    pub fn build(self) -> Result<DeletePaymentMethodDomainResponse, BuildError> {
        Ok(DeletePaymentMethodDomainResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            page_identifier: self
                .page_identifier
                .ok_or_else(|| BuildError::missing_field("page_identifier"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
