pub use crate::prelude::*;

/// Bank account type: Checking or Savings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Achaccounttype {
    Checking,
    Savings,
}
impl fmt::Display for Achaccounttype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Checking => "Checking",
            Self::Savings => "Savings",
        };
        write!(f, "{}", s)
    }
}
