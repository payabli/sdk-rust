pub use crate::prelude::*;

/// Request for AddMethod (body + query parameters)
///
/// Request type for the AddMethodRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddMethodRequest {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "createAnonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_anonymous: Option<CreateAnonymous>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<Temporary>,
    pub body: RequestTokenStorage,
}
