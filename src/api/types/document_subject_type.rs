pub use crate::prelude::*;

/// The record a document is about:
///
/// - `business`: the business.
/// - `paypoint`: the paypoint.
/// - `person`: an owner of the business. Send `personReference` to say who.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocumentSubjectType {
    Business,
    Paypoint,
    Person,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DocumentSubjectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Business => serializer.serialize_str("business"),
            Self::Paypoint => serializer.serialize_str("paypoint"),
            Self::Person => serializer.serialize_str("person"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DocumentSubjectType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "business" => Ok(Self::Business),
            "paypoint" => Ok(Self::Paypoint),
            "person" => Ok(Self::Person),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DocumentSubjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Business => write!(f, "business"),
            Self::Paypoint => write!(f, "paypoint"),
            Self::Person => write!(f, "person"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
