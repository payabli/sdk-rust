pub use crate::prelude::*;

/// Query parameters for refreshPayLinkFromId
///
/// Request type for the RefreshPayLinkFromIdQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefreshPayLinkFromIdQueryRequest {
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_fixed: Option<bool>,
}
