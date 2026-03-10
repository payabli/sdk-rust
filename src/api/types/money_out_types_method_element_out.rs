pub use crate::prelude::*;

/// Configuration for payment method selection on Pay Out payment links.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodElementOut {
    /// Flag indicating if all allowed payment methods will be pre-selected.
    #[serde(rename = "allMethodsChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_methods_checked: Option<bool>,
    /// When `true`, the vendor can select from multiple payment methods. When `false`, only the default method is shown.
    #[serde(rename = "allowMultipleMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_methods: Option<bool>,
    /// The default payment method to highlight on the payment link page. For example, `"vcard"`, `"ach"`, or `"check"`.
    #[serde(rename = "defaultMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_method: Option<String>,
    /// When `true`, the payment methods section is displayed on the payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Header text for the payment methods section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<MethodsListOut>,
    /// Display order of the payment methods section on the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// When `true`, a preview of the virtual card is shown on the payment link page.
    #[serde(rename = "showPreviewVirtualCard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_preview_virtual_card: Option<bool>,
}