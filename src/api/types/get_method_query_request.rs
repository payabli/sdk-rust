pub use crate::prelude::*;

/// Query parameters for GetMethod
///
/// Request type for the GetMethodQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMethodQueryRequest {
    /// Format for card expiration dates in the response.
    ///
    /// Accepted values:
    ///
    /// - 0: default, no formatting. Expiration dates are returned in the format they're saved in.
    ///
    /// - 1: MMYY
    ///
    /// - 2: MM/YY
    #[serde(rename = "cardExpirationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_expiration_format: Option<i64>,
    /// When `true`, the request will include temporary tokens in the search and return details for a matching temporary token. The default behavior searches only for permanent tokens.
    #[serde(rename = "includeTemporary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_temporary: Option<bool>,
}
