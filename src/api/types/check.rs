pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Check {
    /// The checking accountholder's name.
    #[serde(rename = "achHolder")]
    pub ach_holder: AchHolder,
    /// Method to use for the transaction. Use `check` for a paper check transaction. When the method is `check`, then `paymentDetails.checkNumber` is required.
    pub method: String,
}
