pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Finishtype {
    /// Flag to enable 'calendar' option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<bool>,
    /// Flag to enable 'untilCancelled' option
    #[serde(rename = "untilCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_cancelled: Option<bool>,
}
