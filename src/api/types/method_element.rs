pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodElement {
    /// Flag indicating if all allowed payment methods will be pre-selected.
    #[serde(rename = "allMethodsChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_methods_checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<MethodsList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Settings for wallet payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MethodElementSettings>,
}
