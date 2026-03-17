pub use crate::prelude::*;

/// Attachment for a bill.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferOutDetailBillAttachment {
    /// File type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<String>,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File descriptor.
    #[serde(rename = "fileDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_descriptor: Option<String>,
    /// File URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    /// File content.
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}
