pub use crate::prelude::*;

/// The response data containing the result of the import operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseImportResponseData {
    /// The number of records successfully added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    /// List of errors, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// The number of records that were rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
}
