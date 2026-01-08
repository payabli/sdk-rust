pub use crate::prelude::*;

/// The required and recommended fields for a payment made with a stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestSchedulePaymentMethodInitiator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /// Payabli identifier of a tokenized payment method.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<Storedmethodid>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}
