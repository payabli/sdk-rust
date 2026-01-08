pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<CardLinkTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<BasicTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<BasicTable>,
}
