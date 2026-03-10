pub use crate::prelude::*;

/// The status of a payment link.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentLinkStatus {
    Active,
    Expired,
    Canceled,
    Deleted,
}
impl fmt::Display for PaymentLinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Active => "Active",
            Self::Expired => "Expired",
            Self::Canceled => "Canceled",
            Self::Deleted => "Deleted",
        };
        write!(f, "{}", s)
    }
}
