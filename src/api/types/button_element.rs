pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ButtonElement {
    /// Label for custom payment button
    pub label: String,
    /// Specify size of custom payment button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ButtonElementSize>,
}
