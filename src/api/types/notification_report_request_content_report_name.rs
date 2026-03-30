pub use crate::prelude::*;

/// The kind report to generate.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestContentReportName {
    Transaction,
    Settlement,
    Boarding,
    Returned,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationReportRequestContentReportName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transaction => serializer.serialize_str("Transaction"),
            Self::Settlement => serializer.serialize_str("Settlement"),
            Self::Boarding => serializer.serialize_str("Boarding"),
            Self::Returned => serializer.serialize_str("Returned"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationReportRequestContentReportName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Transaction" => Ok(Self::Transaction),
            "Settlement" => Ok(Self::Settlement),
            "Boarding" => Ok(Self::Boarding),
            "Returned" => Ok(Self::Returned),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationReportRequestContentReportName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction => write!(f, "Transaction"),
            Self::Settlement => write!(f, "Settlement"),
            Self::Boarding => write!(f, "Boarding"),
            Self::Returned => write!(f, "Returned"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
