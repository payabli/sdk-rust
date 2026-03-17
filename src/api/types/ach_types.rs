pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AchTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccd: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<BasicTemplateElement>,
}
