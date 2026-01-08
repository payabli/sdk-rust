pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainRequest {
    /// Apple Pay configuration information.
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<AddPaymentMethodDomainRequestApplePay>,
    /// Google Pay configuration information.
    #[serde(rename = "googlePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<AddPaymentMethodDomainRequestGooglePay>,
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<DomainName>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<EntityId>,
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
}
