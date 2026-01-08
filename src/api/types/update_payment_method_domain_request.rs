pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentMethodDomainRequest {
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
    #[serde(rename = "googlePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<UpdatePaymentMethodDomainRequestWallet>,
}
