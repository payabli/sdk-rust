pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefundResponse {
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseData")]
    pub response_data: ResponseDataRefunds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}
