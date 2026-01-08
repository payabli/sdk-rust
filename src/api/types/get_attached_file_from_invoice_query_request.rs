pub use crate::prelude::*;

/// Query parameters for GetAttachedFileFromInvoice
///
/// Request type for the GetAttachedFileFromInvoiceQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttachedFileFromInvoiceQueryRequest {
    /// When `true`, the request returns the file content as a Base64-encoded string.
    #[serde(rename = "returnObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_object: Option<bool>,
}
