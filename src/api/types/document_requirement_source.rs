pub use crate::prelude::*;

/// Where the requirement comes from:
///
/// - `template`: the application's template.
/// - `businessRule`: a Payabli rule, such as bank statements for a business with an annual revenue of $1,000,000 or more.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocumentRequirementSource {
    Template,
    BusinessRule,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DocumentRequirementSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Template => serializer.serialize_str("template"),
            Self::BusinessRule => serializer.serialize_str("businessRule"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DocumentRequirementSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "template" => Ok(Self::Template),
            "businessRule" => Ok(Self::BusinessRule),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DocumentRequirementSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template => write!(f, "template"),
            Self::BusinessRule => write!(f, "businessRule"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
