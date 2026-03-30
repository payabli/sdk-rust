pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KeyValueDuo {
    /// Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl KeyValueDuo {
    pub fn builder() -> KeyValueDuoBuilder {
        <KeyValueDuoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KeyValueDuoBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl KeyValueDuoBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KeyValueDuo`].
    pub fn build(self) -> Result<KeyValueDuo, BuildError> {
        Ok(KeyValueDuo {
            key: self.key,
            value: self.value,
        })
    }
}
