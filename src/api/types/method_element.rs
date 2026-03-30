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
    /// Settings for wallet payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MethodElementSettings>,
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
    settings: Option<MethodElementSettings>,
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

    pub fn settings(mut self, value: MethodElementSettings) -> Self {
        self.settings = Some(value);
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
            settings: self.settings,
        })
    }
}
