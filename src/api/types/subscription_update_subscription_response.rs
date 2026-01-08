pub use crate::prelude::*;

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateSubscriptionResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// If `isSuccess` = true, this contains the identifier of the subscription, and sometimes extra information, depending on what was updated.
    ///
    /// If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
}
