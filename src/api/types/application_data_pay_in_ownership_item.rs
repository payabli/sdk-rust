pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataPayInOwnershipItem {
    #[serde(flatten)]
    pub owners_fields: Owners,
}
