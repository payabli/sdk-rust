pub use crate::prelude::*;

/// The new status to apply to a check payment transaction.
/// - `0`: Cancelled/Voided — Cancels the check transaction.
/// - `5`: Paid — Marks the check transaction as paid.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowedCheckPaymentStatus {
    Cancelled,
    Paid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AllowedCheckPaymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cancelled => serializer.serialize_str("0"),
            Self::Paid => serializer.serialize_str("5"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AllowedCheckPaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0" => Ok(Self::Cancelled),
            "5" => Ok(Self::Paid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AllowedCheckPaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => write!(f, "0"),
            Self::Paid => write!(f, "5"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
