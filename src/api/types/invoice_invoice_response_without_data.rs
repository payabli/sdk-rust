pub use crate::prelude::*;

/// Response schema for invoice operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceResponseWithoutData {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    /// If `isSuccess` = true, this contains the identifier of the invoice. If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    pub response_data: Responsedatanonobject,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    pub room_id: RoomIdNotInUse,
}
