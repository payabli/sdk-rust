pub use crate::prelude::*;

/// Contains details about a file. Max upload size is 30 MB.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileContent {
    /// Content of file, Base64-encoded. Ignored if furl is specified. Max upload size is 30 MB.
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
    /// The name of the attached file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The MIME type of the file (if content is provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<FileContentFtype>,
    /// Optional URL provided to show or download the file remotely
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
}
