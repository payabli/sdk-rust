pub use crate::prelude::*;

/// Bank account changes are currently the only supported case type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CaseType {
    BankAccountChange,
}
impl fmt::Display for CaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BankAccountChange => "BankAccountChange",
        };
        write!(f, "{}", s)
    }
}
