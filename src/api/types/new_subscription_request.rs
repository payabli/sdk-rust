pub use crate::prelude::*;

/// Request for NewSubscription (body + query parameters)
///
/// Request type for the NewSubscriptionRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewSubscriptionRequest {
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: SubscriptionRequestBody,
}
