pub use crate::prelude::*;

/// The new status to apply to a check payment transaction.
/// - `0`: Cancelled/Voided — Cancels the check transaction.
/// - `5`: Paid — Marks the check transaction as paid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AllowedCheckPaymentStatus {
    #[serde(rename = "0")]
    Cancelled,
    #[serde(rename = "5")]
    Paid,
}
impl fmt::Display for AllowedCheckPaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cancelled => "0",
            Self::Paid => "5",
        };
        write!(f, "{}", s)
    }
}
