pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BasicTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<LinkRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<LinkRow>,
}
