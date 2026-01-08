pub use crate::prelude::*;

/// Request for getpaidv2 (body + query parameters)
///
/// Request type for the Getpaidv2Request operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Getpaidv2Request {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    pub body: TransRequestBody,
}
