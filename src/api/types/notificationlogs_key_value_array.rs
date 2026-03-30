pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KeyValueArray {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

impl KeyValueArray {
    pub fn builder() -> KeyValueArrayBuilder {
        <KeyValueArrayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KeyValueArrayBuilder {
    key: Option<String>,
    value: Option<Vec<String>>,
}

impl KeyValueArrayBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: Vec<String>) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KeyValueArray`].
    pub fn build(self) -> Result<KeyValueArray, BuildError> {
        Ok(KeyValueArray {
            key: self.key,
            value: self.value,
        })
    }
}
