pub use crate::prelude::*;

/// Export format for file downloads. When specified, returns data as a file instead of JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExportFormat {
    /// Comma-separated values file
    #[serde(rename = "csv")]
    Csv,
    /// Excel spreadsheet file
    #[serde(rename = "xlsx")]
    Xlsx,
}
impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Csv => "csv",
            Self::Xlsx => "xlsx",
        };
        write!(f, "{}", s)
    }
}
