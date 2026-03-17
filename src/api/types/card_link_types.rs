pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardLinkTypes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<LinkData>,
}
