pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentsRef {
    /// Array of objects describing files contained in the ZIP file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filelist: Option<Vec<PairFiles>>,
    /// Zip file containing attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipfile: Option<String>,
}
