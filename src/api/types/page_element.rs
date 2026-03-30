pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageElement {
    /// Page description in header
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Page header
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl PageElement {
    pub fn builder() -> PageElementBuilder {
        <PageElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageElementBuilder {
    description: Option<String>,
    enabled: Option<Enabled>,
    header: Option<String>,
    order: Option<Order>,
}

impl PageElementBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageElement`].
    pub fn build(self) -> Result<PageElement, BuildError> {
        Ok(PageElement {
            description: self.description,
            enabled: self.enabled,
            header: self.header,
            order: self.order,
        })
    }
}
