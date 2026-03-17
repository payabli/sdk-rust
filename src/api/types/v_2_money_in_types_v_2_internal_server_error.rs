pub use crate::prelude::*;

/// Internal server error response (HTTP 500) returned when an unexpected error occurs. Follows RFC 7807 Problem Details format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V2InternalServerError {
    /// Always "Internal Server Error" for 500 errors.
    pub title: String,
    /// HTTP status code, always 500 for internal errors.
    pub status: i64,
    /// Additional details about the internal error.
    pub detail: String,
    /// Request URL that caused the error.
    pub instance: String,
}
