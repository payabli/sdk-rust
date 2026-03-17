pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<BAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<BDetails>,
}
