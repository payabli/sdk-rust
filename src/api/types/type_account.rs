pub use crate::prelude::*;

/// Type of bank account: Checking or Savings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TypeAccount {
    Checking,
    Savings,
}
impl fmt::Display for TypeAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Checking => "Checking",
            Self::Savings => "Savings",
        };
        write!(f, "{}", s)
    }
}
