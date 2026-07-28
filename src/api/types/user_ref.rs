pub use crate::prelude::*;

/// A reference to a user, with the display name resolved when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserRef {
    /// The user's numeric identifier.
    #[serde(default)]
    pub id: i64,
    /// The user's display name. Null when the name can't be resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UserRef {
    pub fn builder() -> UserRefBuilder {
        <UserRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl UserRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserRefBuilder::id)
    pub fn build(self) -> Result<UserRef, BuildError> {
        Ok(UserRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
