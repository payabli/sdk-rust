pub use crate::prelude::*;

/// Query parameters for SetApprovedBill
///
/// Request type for the SetApprovedBillQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetApprovedBillQueryRequest {
    /// Email or username of user modifying approval status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
