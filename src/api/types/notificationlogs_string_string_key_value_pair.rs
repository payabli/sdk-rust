pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StringStringKeyValuePair {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl StringStringKeyValuePair {
    pub fn builder() -> StringStringKeyValuePairBuilder {
        <StringStringKeyValuePairBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StringStringKeyValuePairBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl StringStringKeyValuePairBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StringStringKeyValuePair`].
    pub fn build(self) -> Result<StringStringKeyValuePair, BuildError> {
        Ok(StringStringKeyValuePair {
            key: self.key,
            value: self.value,
        })
    }
}
