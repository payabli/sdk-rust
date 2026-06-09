pub use crate::prelude::*;

/// Method to use for the transaction. Use `check` for a paper check transaction. When the method is `check`, then `paymentDetails.checkNumber` is required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckMethod {
    #[serde(rename = "check")]
    Check,
}
impl fmt::Display for CheckMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Check => "check",
        };
        write!(f, "{}", s)
    }
}
