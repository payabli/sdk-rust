pub use crate::prelude::*;

/// Response for card validation endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidateResponse {
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: ValidateResponseData,
}

impl ValidateResponse {
    pub fn builder() -> ValidateResponseBuilder {
        <ValidateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateResponseBuilder {
    response_text: Option<ResponseText>,
    is_success: Option<IsSuccess>,
    response_data: Option<ValidateResponseData>,
}

impl ValidateResponseBuilder {
    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: ValidateResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](ValidateResponseBuilder::response_text)
    /// - [`is_success`](ValidateResponseBuilder::is_success)
    /// - [`response_data`](ValidateResponseBuilder::response_data)
    pub fn build(self) -> Result<ValidateResponse, BuildError> {
        Ok(ValidateResponse {
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
