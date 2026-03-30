pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}

impl ApplicationDataPayInContactsItem {
    pub fn builder() -> ApplicationDataPayInContactsItemBuilder {
        <ApplicationDataPayInContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInContactsItemBuilder {
    contacts_fields: Option<Contacts>,
}

impl ApplicationDataPayInContactsItemBuilder {
    pub fn contacts_fields(mut self, value: Contacts) -> Self {
        self.contacts_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayInContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts_fields`](ApplicationDataPayInContactsItemBuilder::contacts_fields)
    pub fn build(self) -> Result<ApplicationDataPayInContactsItem, BuildError> {
        Ok(ApplicationDataPayInContactsItem {
            contacts_fields: self
                .contacts_fields
                .ok_or_else(|| BuildError::missing_field("contacts_fields"))?,
        })
    }
}
