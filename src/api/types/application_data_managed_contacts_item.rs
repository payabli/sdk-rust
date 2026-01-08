pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataManagedContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}
