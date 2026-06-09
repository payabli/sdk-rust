pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliApiResponseOcr {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    /// Details of the OCR processing result
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<OcrResponseData>,
}

impl PayabliApiResponseOcr {
    pub fn builder() -> PayabliApiResponseOcrBuilder {
        <PayabliApiResponseOcrBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseOcrBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_code: Option<Responsecode>,
    response_data: Option<OcrResponseData>,
}

impl PayabliApiResponseOcrBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_data(mut self, value: OcrResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseOcr`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](PayabliApiResponseOcrBuilder::is_success)
    /// - [`response_text`](PayabliApiResponseOcrBuilder::response_text)
    /// - [`response_code`](PayabliApiResponseOcrBuilder::response_code)
    pub fn build(self) -> Result<PayabliApiResponseOcr, BuildError> {
        Ok(PayabliApiResponseOcr {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_data: self.response_data,
        })
    }
}
