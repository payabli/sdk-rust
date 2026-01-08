pub use crate::prelude::*;

/// Response body for payment method deletion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponsePaymethodDelete {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponsePaymethodDeleteResponseData>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
