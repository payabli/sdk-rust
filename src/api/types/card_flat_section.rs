pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardFlatSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CardType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}
