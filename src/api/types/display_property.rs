pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DisplayProperty {
    /// When `true`, the field is displayed on the receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    /// This field is unused.
    #[serde(rename = "Fixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    /// The field's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
