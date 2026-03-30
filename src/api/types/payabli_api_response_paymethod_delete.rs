pub use crate::prelude::*;

/// Response body for payment method deletion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponsePaymethodDelete {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponsePaymethodDeleteResponseData>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponsePaymethodDelete {
    pub fn builder() -> PayabliApiResponsePaymethodDeleteBuilder {
        <PayabliApiResponsePaymethodDeleteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponsePaymethodDeleteBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<PayabliApiResponsePaymethodDeleteResponseData>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponsePaymethodDeleteBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: PayabliApiResponsePaymethodDeleteResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponsePaymethodDelete`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponsePaymethodDeleteBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponsePaymethodDelete, BuildError> {
        Ok(PayabliApiResponsePaymethodDelete {
            is_success: self.is_success,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
