pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OperationResult {
    /// Message describing the result. If the virtual card link was sent successfully, this contains the email address to which the link was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Indicates whether the operation was successful.
    pub success: bool,
}
