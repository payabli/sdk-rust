pub use crate::prelude::*;

/// Response for MoneyIn/void endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoidResponse {
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    pub room_id: RoomIdNotInUse,
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    pub response_data: VoidResponseData,
}
