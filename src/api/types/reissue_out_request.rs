pub use crate::prelude::*;

/// Request for ReissueOut (body + query parameters)
///
/// Request type for the ReissueOutRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReissueOutRequest {
    /// The transaction ID of the payout to reissue.
    #[serde(rename = "transId")]
    pub trans_id: String,
    pub body: ReissuePayoutBody,
}
