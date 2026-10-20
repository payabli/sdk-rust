pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBusinessRequestOperatingSeasonsItem {
    Spring,
    Summer,
    Fall,
    Winter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBusinessRequestOperatingSeasonsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Spring => serializer.serialize_str("Spring"),
            Self::Summer => serializer.serialize_str("Summer"),
            Self::Fall => serializer.serialize_str("Fall"),
            Self::Winter => serializer.serialize_str("Winter"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBusinessRequestOperatingSeasonsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Spring" => Ok(Self::Spring),
            "Summer" => Ok(Self::Summer),
            "Fall" => Ok(Self::Fall),
            "Winter" => Ok(Self::Winter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBusinessRequestOperatingSeasonsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spring => write!(f, "Spring"),
            Self::Summer => write!(f, "Summer"),
            Self::Fall => write!(f, "Fall"),
            Self::Winter => write!(f, "Winter"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
