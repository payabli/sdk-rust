pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<LinkData>>,
}
