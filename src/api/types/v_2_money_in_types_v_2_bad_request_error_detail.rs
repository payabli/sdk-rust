pub use crate::prelude::*;

/// Individual field error detail for bad request responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V2BadRequestErrorDetail {
    /// Description of the validation error.
    pub message: String,
    /// Suggested action to fix the error.
    pub suggestion: String,
}
