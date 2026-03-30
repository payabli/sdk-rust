pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponsePaymentLinks {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    /// If `isSuccess` = true, this contains the payment link identifier. If `isSuccess` = false, this contains the reason of the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponsePaymentLinks {
    pub fn builder() -> PayabliApiResponsePaymentLinksBuilder {
        <PayabliApiResponsePaymentLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponsePaymentLinksBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<String>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponsePaymentLinksBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponsePaymentLinks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](PayabliApiResponsePaymentLinksBuilder::is_success)
    /// - [`response_text`](PayabliApiResponsePaymentLinksBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponsePaymentLinks, BuildError> {
        Ok(PayabliApiResponsePaymentLinks {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
