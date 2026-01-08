pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInBankData {
    #[serde(flatten)]
    pub bank_fields: Bank,
}
