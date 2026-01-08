pub use crate::prelude::*;

/// Bad request error response (HTTP 400) returned when request validation fails. Follows RFC 7807 Problem Details format with additional Payabli-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V2BadRequestError {
    /// A URI reference that identifies the problem type. Points to human-readable documentation for this error type.
    pub r#type: String,
    /// Always "Bad Request" for 400 errors.
    pub title: String,
    /// HTTP status code, always 400 for bad requests.
    pub status: i64,
    /// Short description of the error.
    pub detail: String,
    /// Request URL that caused the error.
    pub instance: String,
    /// Payabli's unified response code for validation errors. Starts with 'E'. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
    pub code: String,
    /// Dictionary of field-specific validation errors. Keys are field paths (e.g., "paymentMethod.cardnumber") and values are arrays of error details.
    pub errors: HashMap<String, Vec<V2BadRequestErrorDetail>>,
    /// Pagination token (equivalent to pageIdentifier in v1 APIs). Usually null for errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
