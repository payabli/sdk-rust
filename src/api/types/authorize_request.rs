pub use crate::prelude::*;

/// Request for Authorize (body + query parameters)
///
/// Request type for the AuthorizeRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizeRequest {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}
