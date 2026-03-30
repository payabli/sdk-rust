pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NoteElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Placeholder text for input field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Pre-populated value for input field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl NoteElement {
    pub fn builder() -> NoteElementBuilder {
        <NoteElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NoteElementBuilder {
    enabled: Option<Enabled>,
    header: Option<String>,
    order: Option<Order>,
    placeholder: Option<String>,
    value: Option<String>,
}

impl NoteElementBuilder {
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

    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NoteElement`].
    pub fn build(self) -> Result<NoteElement, BuildError> {
        Ok(NoteElement {
            enabled: self.enabled,
            header: self.header,
            order: self.order,
            placeholder: self.placeholder,
            value: self.value,
        })
    }
}
