pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardTypePass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<TierItemPass>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<TierItemPass>,
}
