pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataOdpContactsItem {
    #[serde(flatten)]
    pub contacts_fields: Contacts,
}
