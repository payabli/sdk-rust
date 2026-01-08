pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateAdditionalDataSection {
    pub visible: Visible,
    pub fields: HashMap<String, TemplateAdditionalDataField>,
}
