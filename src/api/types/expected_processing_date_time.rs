pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExpectedProcessingDateTime(
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::flexible_datetime::offset::option::deserialize")]
    pub Option<DateTime<FixedOffset>>,
);
