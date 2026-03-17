pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    pub response_text: ResponseText,
}
