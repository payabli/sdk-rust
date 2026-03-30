pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebHeaderParameter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

impl WebHeaderParameter {
    pub fn builder() -> WebHeaderParameterBuilder {
        <WebHeaderParameterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebHeaderParameterBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl WebHeaderParameterBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebHeaderParameter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](WebHeaderParameterBuilder::key)
    /// - [`value`](WebHeaderParameterBuilder::value)
    pub fn build(self) -> Result<WebHeaderParameter, BuildError> {
        Ok(WebHeaderParameter {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
