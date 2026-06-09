pub use crate::prelude::*;

/// Response schema for operations for sending invoices or getting next invoice number.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceNumberResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    /// If `isSuccess` = true, this contains the next available invoice number in the format defined by paypoint settings. If `isSuccess` = false, this contains the reason for the error.
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: String,
}

impl InvoiceNumberResponse {
    pub fn builder() -> InvoiceNumberResponseBuilder {
        <InvoiceNumberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceNumberResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<String>,
}

impl InvoiceNumberResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceNumberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](InvoiceNumberResponseBuilder::is_success)
    /// - [`response_text`](InvoiceNumberResponseBuilder::response_text)
    /// - [`response_data`](InvoiceNumberResponseBuilder::response_data)
    pub fn build(self) -> Result<InvoiceNumberResponse, BuildError> {
        Ok(InvoiceNumberResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
