pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInOwnershipItem {
    #[serde(flatten)]
    pub owners_fields: Owners,
}

impl ApplicationDataPayInOwnershipItem {
    pub fn builder() -> ApplicationDataPayInOwnershipItemBuilder {
        <ApplicationDataPayInOwnershipItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInOwnershipItemBuilder {
    owners_fields: Option<Owners>,
}

impl ApplicationDataPayInOwnershipItemBuilder {
    pub fn owners_fields(mut self, value: Owners) -> Self {
        self.owners_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayInOwnershipItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owners_fields`](ApplicationDataPayInOwnershipItemBuilder::owners_fields)
    pub fn build(self) -> Result<ApplicationDataPayInOwnershipItem, BuildError> {
        Ok(ApplicationDataPayInOwnershipItem {
            owners_fields: self
                .owners_fields
                .ok_or_else(|| BuildError::missing_field("owners_fields"))?,
        })
    }
}
