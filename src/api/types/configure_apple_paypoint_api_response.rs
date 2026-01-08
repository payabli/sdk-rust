pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigureApplePaypointApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<ApplePayPaypointRegistrationData>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    /// Field not in use on this endpoint
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
}
