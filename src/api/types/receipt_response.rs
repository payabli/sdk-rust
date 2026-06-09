pub use crate::prelude::*;

/// Response for SendReceipt endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl ReceiptResponse {
    pub fn builder() -> ReceiptResponseBuilder {
        <ReceiptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptResponseBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_text: Option<ResponseText>,
}

impl ReceiptResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](ReceiptResponseBuilder::response_text)
    pub fn build(self) -> Result<ReceiptResponse, BuildError> {
        Ok(ReceiptResponse {
            is_success: self.is_success,
            page_identifier: self.page_identifier,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
