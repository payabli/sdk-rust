pub use crate::prelude::*;

/// The reason a reviewer denied a case. Required only when denying.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BankReviewDecisionReason {
    CreditDecline,
    FraudDecline,
    KybKycDecline,
    Withdrawn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BankReviewDecisionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreditDecline => serializer.serialize_str("CreditDecline"),
            Self::FraudDecline => serializer.serialize_str("FraudDecline"),
            Self::KybKycDecline => serializer.serialize_str("KybKycDecline"),
            Self::Withdrawn => serializer.serialize_str("Withdrawn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BankReviewDecisionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CreditDecline" => Ok(Self::CreditDecline),
            "FraudDecline" => Ok(Self::FraudDecline),
            "KybKycDecline" => Ok(Self::KybKycDecline),
            "Withdrawn" => Ok(Self::Withdrawn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BankReviewDecisionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreditDecline => write!(f, "CreditDecline"),
            Self::FraudDecline => write!(f, "FraudDecline"),
            Self::KybKycDecline => write!(f, "KybKycDecline"),
            Self::Withdrawn => write!(f, "Withdrawn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
