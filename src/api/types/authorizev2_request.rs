pub use crate::prelude::*;

/// Request for authorizev2 (body + query parameters)
///
/// Request type for the Authorizev2Request operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Authorizev2Request {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}
