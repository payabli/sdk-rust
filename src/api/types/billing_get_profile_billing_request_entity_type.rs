pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetProfileBillingRequestEntityType {
    Organization,
    Paypoint,
    Template,
    Application,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetProfileBillingRequestEntityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Organization => serializer.serialize_str("Organization"),
            Self::Paypoint => serializer.serialize_str("Paypoint"),
            Self::Template => serializer.serialize_str("Template"),
            Self::Application => serializer.serialize_str("Application"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetProfileBillingRequestEntityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Organization" => Ok(Self::Organization),
            "Paypoint" => Ok(Self::Paypoint),
            "Template" => Ok(Self::Template),
            "Application" => Ok(Self::Application),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetProfileBillingRequestEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Organization => write!(f, "Organization"),
            Self::Paypoint => write!(f, "Paypoint"),
            Self::Template => write!(f, "Template"),
            Self::Application => write!(f, "Application"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
