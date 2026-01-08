pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrgXScope {
    #[serde(rename = "orgEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_entry: Option<Orgentryname>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
}
