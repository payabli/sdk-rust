pub use crate::prelude::*;

/// The business ownership type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OwnType {
    LimitedLiabilityCompany,
    NonProfitOrg,
    Partnership,
    PrivateCorp,
    PublicCorp,
    TaxExempt,
    Government,
    SoleProprietor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OwnType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LimitedLiabilityCompany => serializer.serialize_str("Limited Liability Company"),
            Self::NonProfitOrg => serializer.serialize_str("Non-Profit Org"),
            Self::Partnership => serializer.serialize_str("Partnership"),
            Self::PrivateCorp => serializer.serialize_str("Private Corp"),
            Self::PublicCorp => serializer.serialize_str("Public Corp"),
            Self::TaxExempt => serializer.serialize_str("Tax Exempt"),
            Self::Government => serializer.serialize_str("Government"),
            Self::SoleProprietor => serializer.serialize_str("Sole Proprietor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OwnType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Limited Liability Company" => Ok(Self::LimitedLiabilityCompany),
            "Non-Profit Org" => Ok(Self::NonProfitOrg),
            "Partnership" => Ok(Self::Partnership),
            "Private Corp" => Ok(Self::PrivateCorp),
            "Public Corp" => Ok(Self::PublicCorp),
            "Tax Exempt" => Ok(Self::TaxExempt),
            "Government" => Ok(Self::Government),
            "Sole Proprietor" => Ok(Self::SoleProprietor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OwnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitedLiabilityCompany => write!(f, "Limited Liability Company"),
            Self::NonProfitOrg => write!(f, "Non-Profit Org"),
            Self::Partnership => write!(f, "Partnership"),
            Self::PrivateCorp => write!(f, "Private Corp"),
            Self::PublicCorp => write!(f, "Public Corp"),
            Self::TaxExempt => write!(f, "Tax Exempt"),
            Self::Government => write!(f, "Government"),
            Self::SoleProprietor => write!(f, "Sole Proprietor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
