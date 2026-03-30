pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KeyValue {
    /// Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    /// Key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl KeyValue {
    pub fn builder() -> KeyValueBuilder {
        <KeyValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KeyValueBuilder {
    key: Option<String>,
    read_only: Option<ReadOnly>,
    value: Option<String>,
}

impl KeyValueBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn read_only(mut self, value: ReadOnly) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KeyValue`].
    pub fn build(self) -> Result<KeyValue, BuildError> {
        Ok(KeyValue {
            key: self.key,
            read_only: self.read_only,
            value: self.value,
        })
    }
}
