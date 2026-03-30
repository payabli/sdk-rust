pub use crate::prelude::*;

/// Configuration for payment method selection on Pay Out payment links.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl MethodElementOut {
    pub fn builder() -> MethodElementOutBuilder {
        <MethodElementOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodElementOutBuilder {
    all_methods_checked: Option<bool>,
    allow_multiple_methods: Option<bool>,
    default_method: Option<String>,
    enabled: Option<bool>,
    header: Option<String>,
    methods: Option<MethodsListOut>,
    order: Option<i64>,
    show_preview_virtual_card: Option<bool>,
}

impl MethodElementOutBuilder {
    pub fn all_methods_checked(mut self, value: bool) -> Self {
        self.all_methods_checked = Some(value);
        self
    }

    pub fn allow_multiple_methods(mut self, value: bool) -> Self {
        self.allow_multiple_methods = Some(value);
        self
    }

    pub fn default_method(mut self, value: impl Into<String>) -> Self {
        self.default_method = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn methods(mut self, value: MethodsListOut) -> Self {
        self.methods = Some(value);
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn show_preview_virtual_card(mut self, value: bool) -> Self {
        self.show_preview_virtual_card = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodElementOut`].
    pub fn build(self) -> Result<MethodElementOut, BuildError> {
        Ok(MethodElementOut {
            all_methods_checked: self.all_methods_checked,
            allow_multiple_methods: self.allow_multiple_methods,
            default_method: self.default_method,
            enabled: self.enabled,
            header: self.header,
            methods: self.methods,
            order: self.order,
            show_preview_virtual_card: self.show_preview_virtual_card,
        })
    }
}
