pub use crate::prelude::*;

/// Method to use for the transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Methodall {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "cash")]
    Cash,
}
impl fmt::Display for Methodall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
            Self::Ach => "ach",
            Self::Cloud => "cloud",
            Self::Check => "check",
            Self::Cash => "cash",
        };
        write!(f, "{}", s)
    }
}
