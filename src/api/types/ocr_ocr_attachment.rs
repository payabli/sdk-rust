pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "fileDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}
