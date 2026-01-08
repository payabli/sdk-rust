pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigureGooglePaypointApiResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    pub page_identifier: PageIdentifier,
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    #[serde(rename = "responseData")]
    pub response_data: GooglePayPaypointRegistrationData,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    /// Field not in use on this endpoint
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
}
