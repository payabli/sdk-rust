pub use crate::prelude::*;

/// Data related to the payment method domain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMethodDomainApiResponse {
    /// The record type. For payment method domains, this is always `PaymentMethodDomain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "applePay")]
    pub apple_pay: ApplePayData,
    #[serde(rename = "googlePay")]
    pub google_pay: GooglePayData,
    /// Data about the domain's cascade status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascades: Option<Vec<CascadeJobDetails>>,
    #[serde(rename = "createdAt")]
    pub created_at: CreatedAt,
    #[serde(rename = "domainName")]
    pub domain_name: DomainName,
    #[serde(rename = "entityId")]
    pub entity_id: EntityId,
    #[serde(rename = "entityType")]
    pub entity_type: EntityType,
    pub id: PaymentMethodDomainId,
    #[serde(rename = "ownerEntityId")]
    pub owner_entity_id: OwnerEntityId,
    #[serde(rename = "ownerEntityType")]
    pub owner_entity_type: OwnerEntityType,
    #[serde(rename = "updatedAt")]
    pub updated_at: LastModified,
}
