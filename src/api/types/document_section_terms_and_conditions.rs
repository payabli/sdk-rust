pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentSectionTermsAndConditions {
    #[serde(rename = "tcLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_links: Option<Vec<DocumentSectionTermsAndConditionsTcLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}
