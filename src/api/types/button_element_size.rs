pub use crate::prelude::*;

/// Specify size of custom payment button
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ButtonElementSize {
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "lg")]
    Lg,
}
impl fmt::Display for ButtonElementSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        };
        write!(f, "{}", s)
    }
}
