pub use crate::prelude::*;

/// Response for card validation endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidateResponse {
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseData")]
    pub response_data: ValidateResponseData,
}
