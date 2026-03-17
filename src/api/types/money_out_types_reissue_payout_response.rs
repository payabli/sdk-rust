pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReissuePayoutResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    pub response_data: ReissuePayoutResponseData,
}
