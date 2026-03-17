pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AchTypesTiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<TierItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<TierItem>,
}
