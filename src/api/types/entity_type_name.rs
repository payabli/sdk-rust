pub use crate::prelude::*;

/// Entity type, serialized as a name by the List profiles endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityTypeName {
    Organization,
    Paypoint,
    Customer,
    Template,
    Application,
    BankAccount,
    Address,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntityTypeName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Organization => serializer.serialize_str("Organization"),
            Self::Paypoint => serializer.serialize_str("Paypoint"),
            Self::Customer => serializer.serialize_str("Customer"),
            Self::Template => serializer.serialize_str("Template"),
            Self::Application => serializer.serialize_str("Application"),
            Self::BankAccount => serializer.serialize_str("BankAccount"),
            Self::Address => serializer.serialize_str("Address"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntityTypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Organization" => Ok(Self::Organization),
            "Paypoint" => Ok(Self::Paypoint),
            "Customer" => Ok(Self::Customer),
            "Template" => Ok(Self::Template),
            "Application" => Ok(Self::Application),
            "BankAccount" => Ok(Self::BankAccount),
            "Address" => Ok(Self::Address),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntityTypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Organization => write!(f, "Organization"),
            Self::Paypoint => write!(f, "Paypoint"),
            Self::Customer => write!(f, "Customer"),
            Self::Template => write!(f, "Template"),
            Self::Application => write!(f, "Application"),
            Self::BankAccount => write!(f, "BankAccount"),
            Self::Address => write!(f, "Address"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
