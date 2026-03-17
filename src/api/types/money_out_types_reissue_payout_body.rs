pub use crate::prelude::*;

/// Request body for reissuing a payout transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReissuePayoutBody {
    #[serde(rename = "paymentMethod")]
    pub payment_method: ReissuePaymentMethod,
}
