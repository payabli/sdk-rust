pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExportFormat1 {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "xlsx")]
    Xlsx,
}
impl fmt::Display for ExportFormat1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Csv => "csv",
            Self::Xlsx => "xlsx",
        };
        write!(f, "{}", s)
    }
}
