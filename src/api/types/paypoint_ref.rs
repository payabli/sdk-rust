pub use crate::prelude::*;

/// A reference to the paypoint the case applies to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointRef {
    /// The paypoint's numeric identifier.
    #[serde(default)]
    pub id: i64,
    /// The paypoint's DBA name.
    #[serde(default)]
    pub name: String,
}

impl PaypointRef {
    pub fn builder() -> PaypointRefBuilder {
        <PaypointRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl PaypointRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaypointRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaypointRefBuilder::id)
    /// - [`name`](PaypointRefBuilder::name)
    pub fn build(self) -> Result<PaypointRef, BuildError> {
        Ok(PaypointRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
