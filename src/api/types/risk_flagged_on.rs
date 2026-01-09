pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RiskFlaggedOn(
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::flexible_datetime::utc::option::deserialize")]
    pub Option<DateTime<Utc>>,
);
