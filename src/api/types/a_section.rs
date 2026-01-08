pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ASection {
    #[serde(rename = "minimumDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_documents: Option<i64>,
    #[serde(rename = "multipleContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_contacts: Option<bool>,
    #[serde(rename = "multipleOwners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_owners: Option<bool>,
}
