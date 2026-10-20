pub use crate::prelude::*;

/// The business's legal structure, normalized to a canonical value on read.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BusinessDetailResponseLegalStructure {
    LimitedLiabilityCompany,
    Corporation,
    Partnership,
    SoleProprietorship,
    NonProfit,
    Government,
    SCorporation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BusinessDetailResponseLegalStructure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LimitedLiabilityCompany => serializer.serialize_str("LimitedLiabilityCompany"),
            Self::Corporation => serializer.serialize_str("Corporation"),
            Self::Partnership => serializer.serialize_str("Partnership"),
            Self::SoleProprietorship => serializer.serialize_str("SoleProprietorship"),
            Self::NonProfit => serializer.serialize_str("NonProfit"),
            Self::Government => serializer.serialize_str("Government"),
            Self::SCorporation => serializer.serialize_str("SCorporation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BusinessDetailResponseLegalStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LimitedLiabilityCompany" => Ok(Self::LimitedLiabilityCompany),
            "Corporation" => Ok(Self::Corporation),
            "Partnership" => Ok(Self::Partnership),
            "SoleProprietorship" => Ok(Self::SoleProprietorship),
            "NonProfit" => Ok(Self::NonProfit),
            "Government" => Ok(Self::Government),
            "SCorporation" => Ok(Self::SCorporation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BusinessDetailResponseLegalStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitedLiabilityCompany => write!(f, "LimitedLiabilityCompany"),
            Self::Corporation => write!(f, "Corporation"),
            Self::Partnership => write!(f, "Partnership"),
            Self::SoleProprietorship => write!(f, "SoleProprietorship"),
            Self::NonProfit => write!(f, "NonProfit"),
            Self::Government => write!(f, "Government"),
            Self::SCorporation => write!(f, "SCorporation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
