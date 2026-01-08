pub use crate::prelude::*;

/// Request for AddCustomer (body + query parameters)
///
/// Request type for the AddCustomerRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddCustomerRequest {
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<bool>,
    /// Flag indicating to replace existing customer with a new record. Possible values: 0 (don't replace), 1 (replace). Default is `0`.
    #[serde(rename = "replaceExisting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<i64>,
    pub body: CustomerData,
}
