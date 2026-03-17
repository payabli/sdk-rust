pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AchTypesPass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<TierItemPass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<TierItemPass>,
}
