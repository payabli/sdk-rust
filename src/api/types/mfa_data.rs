pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MfaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<bool>,
    #[serde(rename = "mfaMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_mode: Option<MfaMode>,
}
