pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<BasicTemplateElement>,
    #[serde(rename = "masterCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_card: Option<BasicTemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<BasicTemplateElement>,
}
