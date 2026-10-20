pub use crate::prelude::*;

/// What a payment method is used for. One method can carry several usages.
///
/// - `deposits` — Pay In deposits
/// - `billing` — Billing
/// - `withdrawals` — Withdrawals
/// - `payOutFunding` — Pay Out funding and remittances
/// - `refunds` — Refunds
/// - `returns` — Returns
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentMethodUsage {
    Deposits,
    Billing,
    Withdrawals,
    PayOutFunding,
    Refunds,
    Returns,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentMethodUsage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Deposits => serializer.serialize_str("deposits"),
            Self::Billing => serializer.serialize_str("billing"),
            Self::Withdrawals => serializer.serialize_str("withdrawals"),
            Self::PayOutFunding => serializer.serialize_str("payOutFunding"),
            Self::Refunds => serializer.serialize_str("refunds"),
            Self::Returns => serializer.serialize_str("returns"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentMethodUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "deposits" => Ok(Self::Deposits),
            "billing" => Ok(Self::Billing),
            "withdrawals" => Ok(Self::Withdrawals),
            "payOutFunding" => Ok(Self::PayOutFunding),
            "refunds" => Ok(Self::Refunds),
            "returns" => Ok(Self::Returns),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentMethodUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deposits => write!(f, "deposits"),
            Self::Billing => write!(f, "billing"),
            Self::Withdrawals => write!(f, "withdrawals"),
            Self::PayOutFunding => write!(f, "payOutFunding"),
            Self::Refunds => write!(f, "refunds"),
            Self::Returns => write!(f, "returns"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
