pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayorElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Array of Customer/Payor fields to show in section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<PayorFields>>,
    /// Custom header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

impl PayorElement {
    pub fn builder() -> PayorElementBuilder {
        <PayorElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayorElementBuilder {
    enabled: Option<Enabled>,
    fields: Option<Vec<PayorFields>>,
    header: Option<String>,
    order: Option<Order>,
}

impl PayorElementBuilder {
    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<PayorFields>) -> Self {
        self.fields = Some(value);
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

    /// Consumes the builder and constructs a [`PayorElement`].
    pub fn build(self) -> Result<PayorElement, BuildError> {
        Ok(PayorElement {
            enabled: self.enabled,
            fields: self.fields,
            header: self.header,
            order: self.order,
        })
    }
}
