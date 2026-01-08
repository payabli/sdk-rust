pub use crate::prelude::*;

/// Describes whether the bank is a personal or business account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BankAccountHolderType {
    Personal,
    Business,
}
impl fmt::Display for BankAccountHolderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Personal => "Personal",
            Self::Business => "Business",
        };
        write!(f, "{}", s)
    }
}
