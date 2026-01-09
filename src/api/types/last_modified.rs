pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LastModified(
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::flexible_datetime::utc::option::deserialize")]
    pub Option<DateTime<Utc>>,
);
