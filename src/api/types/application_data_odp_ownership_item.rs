pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDataOdpOwnershipItem(pub Owners);
