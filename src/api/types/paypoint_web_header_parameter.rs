pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebHeaderParameter {
    pub key: String,
    pub value: String,
}
