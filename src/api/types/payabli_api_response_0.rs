pub use crate::prelude::*;

/// General response for certain `moneyIn` and `moneyOut` endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse0 {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponse0ResponseData>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
