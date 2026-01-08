pub use crate::prelude::*;

/// Response for MoneyIn/authorize.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthResponse {
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseData")]
    pub response_data: AuthResponseResponseData,
}
