pub use crate::prelude::*;

/// Response schema for line item operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse6 {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    /// If `isSuccess` = true, this contains the line item identifier. If `isSuccess` = false, this contains the reason of the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<Responsedatanonobject>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
