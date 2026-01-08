pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Replyby(
    #[serde(deserialize_with = "crate::core::flexible_datetime::offset::deserialize")]
    pub  DateTime<FixedOffset>,
);
