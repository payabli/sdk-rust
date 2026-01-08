pub use crate::prelude::*;

/// Request for getpaid (body + query parameters)
///
/// Request type for the GetpaidRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetpaidRequest {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    /// When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
    #[serde(rename = "includeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_details: Option<bool>,
    pub body: TransRequestBody,
}
