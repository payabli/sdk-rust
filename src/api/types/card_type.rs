pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<TierItem>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<TierItem>,
}
