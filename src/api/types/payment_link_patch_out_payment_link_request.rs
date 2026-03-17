pub use crate::prelude::*;

/// Request body for partially updating a Pay Out payment link.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PatchOutPaymentLinkRequest {
    /// Updated payment link page configuration.
    #[serde(rename = "billPageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_page_data: Option<PaymentPageRequestBodyOut>,
    /// New expiration date for the payment link. Must be a future date. If null and the link is expired, uses the default expiration from settings. Updating the expiration date reactivates an expired payment link to Active status.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Updated status for the payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentLinkStatus>,
}
