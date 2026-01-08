pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Cash {
    /// Method to use for the transaction. For cash transactions, use `cash`.
    pub method: String,
}
