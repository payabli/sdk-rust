pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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
    /// Description text shown in the payment methods section.
    #[serde(rename = "paymentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
    /// Settings for wallet payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MethodElementSettings>,
    /// Controls whether the "Save payment details for future use" checkbox appears on the hosted payment page. Set to `false` to hide the checkbox. Defaults to `true`.
    #[serde(rename = "showSaveMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_save_method: Option<bool>,
}

impl MethodElement {
    pub fn builder() -> MethodElementBuilder {
        <MethodElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodElementBuilder {
    all_methods_checked: Option<bool>,
    enabled: Option<Enabled>,
    header: Option<String>,
    methods: Option<MethodsList>,
    order: Option<Order>,
    payment_description: Option<String>,
    settings: Option<MethodElementSettings>,
    show_save_method: Option<bool>,
}

impl MethodElementBuilder {
    pub fn all_methods_checked(mut self, value: bool) -> Self {
        self.all_methods_checked = Some(value);
        self
    }

    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn methods(mut self, value: MethodsList) -> Self {
        self.methods = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn payment_description(mut self, value: impl Into<String>) -> Self {
        self.payment_description = Some(value.into());
        self
    }

    pub fn settings(mut self, value: MethodElementSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn show_save_method(mut self, value: bool) -> Self {
        self.show_save_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodElement`].
    pub fn build(self) -> Result<MethodElement, BuildError> {
        Ok(MethodElement {
            all_methods_checked: self.all_methods_checked,
            enabled: self.enabled,
            header: self.header,
            methods: self.methods,
            order: self.order,
            payment_description: self.payment_description,
            settings: self.settings,
            show_save_method: self.show_save_method,
        })
    }
}
