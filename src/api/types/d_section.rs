pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DSection {
    #[serde(rename = "depositAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_account: Option<Bnk>,
    #[serde(rename = "withdrawalAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_account: Option<Bnk>,
}
