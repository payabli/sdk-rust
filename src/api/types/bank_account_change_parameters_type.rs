pub use crate::prelude::*;

/// The parameters type discriminator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BankAccountChangeParametersType {
    BankAccountChange,
}
impl fmt::Display for BankAccountChangeParametersType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BankAccountChange => "BankAccountChange",
        };
        write!(f, "{}", s)
    }
}
