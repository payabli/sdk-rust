pub use crate::prelude::*;

/// The business's legal structure. Case-insensitive, and several inputs normalize to the same canonical value on read (for example, `llc` and `limited-liability-company` both read back as `LimitedLiabilityCompany`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBusinessRequestLegalStructure {
    LimitedLiabilityCompany,
    Llc,
    Corporation,
    Partnership,
    SoleProprietorship,
    NonProfit,
    Government,
    SCorp,
    SCorporation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBusinessRequestLegalStructure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LimitedLiabilityCompany => serializer.serialize_str("limited-liability-company"),
            Self::Llc => serializer.serialize_str("llc"),
            Self::Corporation => serializer.serialize_str("corporation"),
            Self::Partnership => serializer.serialize_str("partnership"),
            Self::SoleProprietorship => serializer.serialize_str("sole-proprietorship"),
            Self::NonProfit => serializer.serialize_str("non-profit"),
            Self::Government => serializer.serialize_str("government"),
            Self::SCorp => serializer.serialize_str("s-corp"),
            Self::SCorporation => serializer.serialize_str("s-corporation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBusinessRequestLegalStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "limited-liability-company" => Ok(Self::LimitedLiabilityCompany),
            "llc" => Ok(Self::Llc),
            "corporation" => Ok(Self::Corporation),
            "partnership" => Ok(Self::Partnership),
            "sole-proprietorship" => Ok(Self::SoleProprietorship),
            "non-profit" => Ok(Self::NonProfit),
            "government" => Ok(Self::Government),
            "s-corp" => Ok(Self::SCorp),
            "s-corporation" => Ok(Self::SCorporation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBusinessRequestLegalStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitedLiabilityCompany => write!(f, "limited-liability-company"),
            Self::Llc => write!(f, "llc"),
            Self::Corporation => write!(f, "corporation"),
            Self::Partnership => write!(f, "partnership"),
            Self::SoleProprietorship => write!(f, "sole-proprietorship"),
            Self::NonProfit => write!(f, "non-profit"),
            Self::Government => write!(f, "government"),
            Self::SCorp => write!(f, "s-corp"),
            Self::SCorporation => write!(f, "s-corporation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
