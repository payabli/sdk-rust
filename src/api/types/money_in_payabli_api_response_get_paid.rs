pub use crate::prelude::*;

/// General response for GetPaid endpoint supporting multiple payment methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliApiResponseGetPaid {
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<String>,
    #[serde(rename = "responseData")]
    pub response_data: GetPaidResponseData,
}
