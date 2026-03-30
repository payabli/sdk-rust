pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ButtonElement {
    /// Label for custom payment button
    #[serde(default)]
    pub label: String,
    /// Specify size of custom payment button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ButtonElementSize>,
}

impl ButtonElement {
    pub fn builder() -> ButtonElementBuilder {
        <ButtonElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ButtonElementBuilder {
    label: Option<String>,
    size: Option<ButtonElementSize>,
}

impl ButtonElementBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn size(mut self, value: ButtonElementSize) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ButtonElement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ButtonElementBuilder::label)
    pub fn build(self) -> Result<ButtonElement, BuildError> {
        Ok(ButtonElement {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            size: self.size,
        })
    }
}
