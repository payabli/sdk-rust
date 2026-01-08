pub use crate::prelude::*;

/// Request for UpdateMethod (body + query parameters)
///
/// Request type for the UpdateMethodRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateMethodRequest {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    pub body: RequestTokenStorage,
}
