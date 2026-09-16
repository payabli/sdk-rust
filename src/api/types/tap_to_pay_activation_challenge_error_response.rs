pub use crate::prelude::*;

/// Error response for a Tap to Pay activation challenge request.
/// `responseData` carries the same status code as the HTTP response
/// and a message describing the refusal.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TapToPayActivationChallengeErrorResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: TapToPayActivationChallengeErrorResponseResponseData,
}

impl TapToPayActivationChallengeErrorResponse {
    pub fn builder() -> TapToPayActivationChallengeErrorResponseBuilder {
        <TapToPayActivationChallengeErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TapToPayActivationChallengeErrorResponseBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<TapToPayActivationChallengeErrorResponseResponseData>,
}

impl TapToPayActivationChallengeErrorResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(
        mut self,
        value: TapToPayActivationChallengeErrorResponseResponseData,
    ) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TapToPayActivationChallengeErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](TapToPayActivationChallengeErrorResponseBuilder::is_success)
    /// - [`response_text`](TapToPayActivationChallengeErrorResponseBuilder::response_text)
    /// - [`response_data`](TapToPayActivationChallengeErrorResponseBuilder::response_data)
    pub fn build(self) -> Result<TapToPayActivationChallengeErrorResponse, BuildError> {
        Ok(TapToPayActivationChallengeErrorResponse {
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
