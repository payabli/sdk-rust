pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SendVCardLinkRequest {
    /// The transaction ID of the virtual card payout. The ID is returned as `ReferenceId` in the response when you authorize a payout with POST /MoneyOut/authorize.
    #[serde(rename = "transId")]
    pub trans_id: String,
}
