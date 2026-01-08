pub use crate::prelude::*;

/// Query parameters for deleteAttachedFromBill
///
/// Request type for the DeleteAttachedFromBillQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAttachedFromBillQueryRequest {
    /// When `true`, the request returns the file content as a Base64-encoded string.
    #[serde(rename = "returnObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_object: Option<bool>,
}
