pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAuthPswResetRequest {
    /// New User password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psw: Option<String>,
}
