pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list: Option<CList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_list: Option<OList>,
}
