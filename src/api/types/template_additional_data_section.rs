pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateAdditionalDataSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    pub fields: HashMap<String, TemplateAdditionalDataField>,
}