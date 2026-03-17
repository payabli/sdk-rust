pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddSubscriptionResponse {
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// The identifier of the newly created subscription.
    #[serde(rename = "responseData")]
    pub response_data: i64,
}
