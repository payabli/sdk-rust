pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferMessageProperties {
    #[serde(rename = "originalTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transfer_status: Option<String>,
    #[serde(rename = "currentTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_transfer_status: Option<String>,
}
