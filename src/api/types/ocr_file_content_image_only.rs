pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileContentImageOnly {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<FileContentFtype>,
    /// The name of the file to be uploaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Optional URL link to the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    /// Base64-encoded file content
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}
