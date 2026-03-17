pub use crate::prelude::*;

/// Details about pricing and payment services for a business.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServicesSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardService>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}
