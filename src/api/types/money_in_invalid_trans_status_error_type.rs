pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvalidTransStatusErrorType {
    /// Error message describing the reason for the decline
    #[serde(rename = "responseText")]
    pub response_text: String,
}
