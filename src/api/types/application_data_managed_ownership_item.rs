pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataManagedOwnershipItem {
    #[serde(flatten)]
    pub owners_fields: Owners,
}

impl ApplicationDataManagedOwnershipItem {
    pub fn builder() -> ApplicationDataManagedOwnershipItemBuilder {
        <ApplicationDataManagedOwnershipItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataManagedOwnershipItemBuilder {
    owners_fields: Option<Owners>,
}

impl ApplicationDataManagedOwnershipItemBuilder {
    pub fn owners_fields(mut self, value: Owners) -> Self {
        self.owners_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataManagedOwnershipItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owners_fields`](ApplicationDataManagedOwnershipItemBuilder::owners_fields)
    pub fn build(self) -> Result<ApplicationDataManagedOwnershipItem, BuildError> {
        Ok(ApplicationDataManagedOwnershipItem {
            owners_fields: self
                .owners_fields
                .ok_or_else(|| BuildError::missing_field("owners_fields"))?,
        })
    }
}
