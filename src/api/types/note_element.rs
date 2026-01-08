pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NoteElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Placeholder text for input field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Pre-populated value for input field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
