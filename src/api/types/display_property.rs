pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisplayProperty {
    /// When `true`, the field is displayed on the receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    /// This field is unused.
    #[serde(rename = "Fixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    /// The field's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DisplayProperty {
    pub fn builder() -> DisplayPropertyBuilder {
        <DisplayPropertyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisplayPropertyBuilder {
    display: Option<bool>,
    fixed: Option<bool>,
    name: Option<String>,
}

impl DisplayPropertyBuilder {
    pub fn display(mut self, value: bool) -> Self {
        self.display = Some(value);
        self
    }

    pub fn fixed(mut self, value: bool) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisplayProperty`].
    pub fn build(self) -> Result<DisplayProperty, BuildError> {
        Ok(DisplayProperty {
            display: self.display,
            fixed: self.fixed,
            name: self.name,
        })
    }
}
