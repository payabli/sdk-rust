pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDataOdpContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}

impl ApplicationDataOdpContactsItem {
    pub fn builder() -> ApplicationDataOdpContactsItemBuilder {
        <ApplicationDataOdpContactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataOdpContactsItemBuilder {
    contacts_fields: Option<Contacts>,
}

impl ApplicationDataOdpContactsItemBuilder {
    pub fn contacts_fields(mut self, value: Contacts) -> Self {
        self.contacts_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataOdpContactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts_fields`](ApplicationDataOdpContactsItemBuilder::contacts_fields)
    pub fn build(self) -> Result<ApplicationDataOdpContactsItem, BuildError> {
        Ok(ApplicationDataOdpContactsItem {
            contacts_fields: self
                .contacts_fields
                .ok_or_else(|| BuildError::missing_field("contacts_fields"))?,
        })
    }
}
