pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PairFiles {
    /// Original filename
    #[serde(rename = "originalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    /// Filename assigned to zipped file. This is the name to use for reference in the API functions to get files in attachments.
    #[serde(rename = "zipName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_name: Option<String>,
    /// Descriptor of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
}
