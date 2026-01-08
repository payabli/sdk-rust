pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponsePaymentLinks {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    /// If `isSuccess` = true, this contains the payment link identifier. If `isSuccess` = false, this contains the reason of the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
