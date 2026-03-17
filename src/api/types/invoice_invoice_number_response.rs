pub use crate::prelude::*;

/// Response schema for operations for sending invoices or getting next invoice number.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceNumberResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    /// If `isSuccess` = true, this contains the next available invoice number in the format defined by paypoint settings. If `isSuccess` = false, this contains the reason for the error.
    #[serde(rename = "responseData")]
    pub response_data: String,
}
