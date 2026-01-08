pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AchPassThroughSection {
    #[serde(rename = "multiTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<AchTypesPass>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}
