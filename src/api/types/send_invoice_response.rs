pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendInvoiceResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl SendInvoiceResponse {
    pub fn builder() -> SendInvoiceResponseBuilder {
        <SendInvoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendInvoiceResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
}

impl SendInvoiceResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendInvoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](SendInvoiceResponseBuilder::is_success)
    /// - [`response_text`](SendInvoiceResponseBuilder::response_text)
    pub fn build(self) -> Result<SendInvoiceResponse, BuildError> {
        Ok(SendInvoiceResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
