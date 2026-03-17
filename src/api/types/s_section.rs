pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardSection>,
}
