pub use crate::prelude::*;

/// Apple Pay configuration information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainRequestApplePay {
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}
