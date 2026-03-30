pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HeaderElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl HeaderElement {
    pub fn builder() -> HeaderElementBuilder {
        <HeaderElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HeaderElementBuilder {
    enabled: Option<Enabled>,
    header: Option<String>,
    order: Option<Order>,
}

impl HeaderElementBuilder {
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

    /// Consumes the builder and constructs a [`HeaderElement`].
    pub fn build(self) -> Result<HeaderElement, BuildError> {
        Ok(HeaderElement {
            enabled: self.enabled,
            header: self.header,
            order: self.order,
        })
    }
}
