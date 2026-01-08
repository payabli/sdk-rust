pub use crate::prelude::*;

/// Request for SendToApprovalBill (body + query parameters)
///
/// Request type for the SendToApprovalBillRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SendToApprovalBillRequest {
    /// Automatically create the target user for approval if they don't exist.
    #[serde(rename = "autocreateUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocreate_user: Option<bool>,
    pub body: Vec<String>,
}
