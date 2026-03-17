pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDataOdpOwnershipItem {
    #[serde(flatten)]
    pub owners_fields: Owners,
}
