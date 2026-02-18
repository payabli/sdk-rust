pub use crate::prelude::*;

/// Properties associated with a transfer message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferOutMessageProperties {
    /// The original status of the transfer before the message.
    #[serde(rename = "originalTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transfer_status: Option<String>,
    /// The current status of the transfer after the message.
    #[serde(rename = "currentTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_transfer_status: Option<String>,
}