pub use crate::prelude::*;

/// The bank's accountholder type: personal or business.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AchHolderType {
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "business")]
    Business,
}
impl fmt::Display for AchHolderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Personal => "personal",
            Self::Business => "business",
        };
        write!(f, "{}", s)
    }
}
