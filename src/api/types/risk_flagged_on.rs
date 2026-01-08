pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RiskFlaggedOn(
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::flexible_datetime::offset::option::deserialize")]
    pub Option<DateTime<FixedOffset>>,
);
