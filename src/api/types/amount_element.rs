pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AmountElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PayCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl AmountElement {
    pub fn builder() -> AmountElementBuilder {
        <AmountElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AmountElementBuilder {
    categories: Option<Vec<PayCategory>>,
    enabled: Option<Enabled>,
    order: Option<Order>,
}

impl AmountElementBuilder {
    pub fn categories(mut self, value: Vec<PayCategory>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AmountElement`].
    pub fn build(self) -> Result<AmountElement, BuildError> {
        Ok(AmountElement {
            categories: self.categories,
            enabled: self.enabled,
            order: self.order,
        })
    }
}
