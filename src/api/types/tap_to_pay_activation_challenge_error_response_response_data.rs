pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TapToPayActivationChallengeErrorResponseResponseData {
    /// The same status code as the HTTP response.
    #[serde(rename = "resultCode")]
    #[serde(default)]
    pub result_code: i64,
    /// A message describing why the request was refused.
    #[serde(rename = "resultText")]
    #[serde(default)]
    pub result_text: String,
}

impl TapToPayActivationChallengeErrorResponseResponseData {
    pub fn builder() -> TapToPayActivationChallengeErrorResponseResponseDataBuilder {
        <TapToPayActivationChallengeErrorResponseResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TapToPayActivationChallengeErrorResponseResponseDataBuilder {
    result_code: Option<i64>,
    result_text: Option<String>,
}

impl TapToPayActivationChallengeErrorResponseResponseDataBuilder {
    pub fn result_code(mut self, value: i64) -> Self {
        self.result_code = Some(value);
        self
    }

    pub fn result_text(mut self, value: impl Into<String>) -> Self {
        self.result_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TapToPayActivationChallengeErrorResponseResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result_code`](TapToPayActivationChallengeErrorResponseResponseDataBuilder::result_code)
    /// - [`result_text`](TapToPayActivationChallengeErrorResponseResponseDataBuilder::result_text)
    pub fn build(self) -> Result<TapToPayActivationChallengeErrorResponseResponseData, BuildError> {
        Ok(TapToPayActivationChallengeErrorResponseResponseData {
            result_code: self
                .result_code
                .ok_or_else(|| BuildError::missing_field("result_code"))?,
            result_text: self
                .result_text
                .ok_or_else(|| BuildError::missing_field("result_text"))?,
        })
    }
}
