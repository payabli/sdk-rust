pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TemplateAdditionalDataField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<RequiredElement>,
    #[serde(rename = "posRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_row: Option<PosRow>,
    #[serde(rename = "posCol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_col: Option<PosCol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueTemplates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
