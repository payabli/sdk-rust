pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataManagedContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}

impl ApplicationDataManagedContactsItem {
    pub fn builder() -> ApplicationDataManagedContactsItemBuilder {
        <ApplicationDataManagedContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataManagedContactsItemBuilder {
    contacts_fields: Option<Contacts>,
}

impl ApplicationDataManagedContactsItemBuilder {
    pub fn contacts_fields(mut self, value: Contacts) -> Self {
        self.contacts_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataManagedContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts_fields`](ApplicationDataManagedContactsItemBuilder::contacts_fields)
    pub fn build(self) -> Result<ApplicationDataManagedContactsItem, BuildError> {
        Ok(ApplicationDataManagedContactsItem {
            contacts_fields: self
                .contacts_fields
                .ok_or_else(|| BuildError::missing_field("contacts_fields"))?,
        })
    }
}
