pub use crate::prelude::*;

/// A reference to the organization that owns the case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrgRef {
    /// The organization's numeric identifier.
    #[serde(default)]
    pub id: i64,
    /// The organization's name.
    #[serde(default)]
    pub name: String,
}

impl OrgRef {
    pub fn builder() -> OrgRefBuilder {
        <OrgRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrgRefBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl OrgRefBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrgRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrgRefBuilder::id)
    /// - [`name`](OrgRefBuilder::name)
    pub fn build(self) -> Result<OrgRef, BuildError> {
        Ok(OrgRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
