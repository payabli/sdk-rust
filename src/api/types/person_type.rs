pub use crate::prelude::*;

/// The role a person plays on a business. Applies only when the person is linked to a business.
/// Application validation holds every role except `Employee` to the owner requirements.
/// Link anyone who doesn't own part of the business, such as a contact, as an `Employee`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersonType {
    Owner,
    Director,
    Contact,
    Signer,
    AuthorizedUser,
    Employee,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PersonType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("Owner"),
            Self::Director => serializer.serialize_str("Director"),
            Self::Contact => serializer.serialize_str("Contact"),
            Self::Signer => serializer.serialize_str("Signer"),
            Self::AuthorizedUser => serializer.serialize_str("AuthorizedUser"),
            Self::Employee => serializer.serialize_str("Employee"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PersonType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Owner" => Ok(Self::Owner),
            "Director" => Ok(Self::Director),
            "Contact" => Ok(Self::Contact),
            "Signer" => Ok(Self::Signer),
            "AuthorizedUser" => Ok(Self::AuthorizedUser),
            "Employee" => Ok(Self::Employee),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PersonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "Owner"),
            Self::Director => write!(f, "Director"),
            Self::Contact => write!(f, "Contact"),
            Self::Signer => write!(f, "Signer"),
            Self::AuthorizedUser => write!(f, "AuthorizedUser"),
            Self::Employee => write!(f, "Employee"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
