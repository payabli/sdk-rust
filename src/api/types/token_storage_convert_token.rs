pub use crate::prelude::*;

/// Object containing the information needed to convert a temporary token to a permanent token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConvertToken {
    /// The type of payment method to tokenize. When converting a temp token to a permanent token, this should match the `method` set for the temporary token, either `ach` or `card`.
    pub method: String,
    /// A temporary stored token ID to be converted to permanent.
    #[serde(rename = "tokenId")]
    pub token_id: String,
}
