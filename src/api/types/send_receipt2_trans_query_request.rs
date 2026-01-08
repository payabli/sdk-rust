pub use crate::prelude::*;

/// Query parameters for SendReceipt2Trans
///
/// Request type for the SendReceipt2TransQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendReceipt2TransQueryRequest {
    /// Email address where the payment receipt should be sent.
    ///
    /// If not provided, the email address on file for the user owner of the transaction is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
