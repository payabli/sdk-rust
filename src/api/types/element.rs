pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Element {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl Element {
    pub fn builder() -> ElementBuilder {
        <ElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElementBuilder {
    enabled: Option<Enabled>,
    order: Option<Order>,
}

impl ElementBuilder {
    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Element`].
    pub fn build(self) -> Result<Element, BuildError> {
        Ok(Element {
            enabled: self.enabled,
            order: self.order,
        })
    }
}
