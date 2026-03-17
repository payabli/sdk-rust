pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UsrAccess {
    #[serde(rename = "roleLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_label: Option<String>,
    #[serde(rename = "roleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_value: Option<bool>,
}
