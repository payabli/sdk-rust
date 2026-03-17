pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AchLinkTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<LinkData>,
}
