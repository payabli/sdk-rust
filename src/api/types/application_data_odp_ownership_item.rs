pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataOdpOwnershipItem {
    #[serde(flatten)]
    pub owners_fields: Owners,
}

impl ApplicationDataOdpOwnershipItem {
    pub fn builder() -> ApplicationDataOdpOwnershipItemBuilder {
        <ApplicationDataOdpOwnershipItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataOdpOwnershipItemBuilder {
    owners_fields: Option<Owners>,
}

impl ApplicationDataOdpOwnershipItemBuilder {
    pub fn owners_fields(mut self, value: Owners) -> Self {
        self.owners_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataOdpOwnershipItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owners_fields`](ApplicationDataOdpOwnershipItemBuilder::owners_fields)
    pub fn build(self) -> Result<ApplicationDataOdpOwnershipItem, BuildError> {
        Ok(ApplicationDataOdpOwnershipItem {
            owners_fields: self
                .owners_fields
                .ok_or_else(|| BuildError::missing_field("owners_fields"))?,
        })
    }
}
