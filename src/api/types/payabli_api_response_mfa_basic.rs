pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseMfaBasic {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<Mfa>,
    /// The mode of multi-factor authentication used.
    #[serde(rename = "mfaMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_mode: Option<String>,
    #[serde(rename = "mfaValidationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_validation_code: Option<MfaValidationCode>,
    /// Data returned by the response, masked for security.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponseMfaBasic {
    pub fn builder() -> PayabliApiResponseMfaBasicBuilder {
        <PayabliApiResponseMfaBasicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseMfaBasicBuilder {
    is_success: Option<IsSuccess>,
    mfa: Option<Mfa>,
    mfa_mode: Option<String>,
    mfa_validation_code: Option<MfaValidationCode>,
    response_data: Option<String>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponseMfaBasicBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn mfa(mut self, value: Mfa) -> Self {
        self.mfa = Some(value);
        self
    }

    pub fn mfa_mode(mut self, value: impl Into<String>) -> Self {
        self.mfa_mode = Some(value.into());
        self
    }

    pub fn mfa_validation_code(mut self, value: MfaValidationCode) -> Self {
        self.mfa_validation_code = Some(value);
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

    /// Consumes the builder and constructs a [`PayabliApiResponseMfaBasic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponseMfaBasicBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponseMfaBasic, BuildError> {
        Ok(PayabliApiResponseMfaBasic {
            is_success: self.is_success,
            mfa: self.mfa,
            mfa_mode: self.mfa_mode,
            mfa_validation_code: self.mfa_validation_code,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
