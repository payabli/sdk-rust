pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliApiResponsePaylinks {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    /// The paylink ID or error details.
    #[serde(rename = "responseData")]
    pub response_data: Responsedata,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
