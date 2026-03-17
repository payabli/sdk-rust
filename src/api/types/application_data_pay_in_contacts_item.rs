pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}
