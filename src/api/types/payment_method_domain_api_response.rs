pub use crate::prelude::*;

/// Data related to the payment method domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDomainApiResponse {
    /// The record type. For payment method domains, this is always `PaymentMethodDomain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "applePay")]
    #[serde(default)]
    pub apple_pay: ApplePayData,
    #[serde(rename = "googlePay")]
    #[serde(default)]
    pub google_pay: GooglePayData,
    /// Data about the domain's cascade status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascades: Option<Vec<CascadeJobDetails>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: CreatedAt,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: DomainName,
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: EntityId,
    #[serde(rename = "entityType")]
    #[serde(default)]
    pub entity_type: EntityType,
    #[serde(default)]
    pub id: PaymentMethodDomainId,
    #[serde(rename = "ownerEntityId")]
    #[serde(default)]
    pub owner_entity_id: OwnerEntityId,
    #[serde(rename = "ownerEntityType")]
    #[serde(default)]
    pub owner_entity_type: OwnerEntityType,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
}

impl PaymentMethodDomainApiResponse {
    pub fn builder() -> PaymentMethodDomainApiResponseBuilder {
        <PaymentMethodDomainApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDomainApiResponseBuilder {
    r#type: Option<String>,
    apple_pay: Option<ApplePayData>,
    google_pay: Option<GooglePayData>,
    cascades: Option<Vec<CascadeJobDetails>>,
    created_at: Option<CreatedAt>,
    domain_name: Option<DomainName>,
    entity_id: Option<EntityId>,
    entity_type: Option<EntityType>,
    id: Option<PaymentMethodDomainId>,
    owner_entity_id: Option<OwnerEntityId>,
    owner_entity_type: Option<OwnerEntityType>,
    updated_at: Option<LastModified>,
}

impl PaymentMethodDomainApiResponseBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn apple_pay(mut self, value: ApplePayData) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn google_pay(mut self, value: GooglePayData) -> Self {
        self.google_pay = Some(value);
        self
    }

    pub fn cascades(mut self, value: Vec<CascadeJobDetails>) -> Self {
        self.cascades = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn domain_name(mut self, value: DomainName) -> Self {
        self.domain_name = Some(value);
        self
    }

    pub fn entity_id(mut self, value: EntityId) -> Self {
        self.entity_id = Some(value);
        self
    }

    pub fn entity_type(mut self, value: EntityType) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn id(mut self, value: PaymentMethodDomainId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn owner_entity_id(mut self, value: OwnerEntityId) -> Self {
        self.owner_entity_id = Some(value);
        self
    }

    pub fn owner_entity_type(mut self, value: OwnerEntityType) -> Self {
        self.owner_entity_type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: LastModified) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDomainApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`apple_pay`](PaymentMethodDomainApiResponseBuilder::apple_pay)
    /// - [`google_pay`](PaymentMethodDomainApiResponseBuilder::google_pay)
    /// - [`created_at`](PaymentMethodDomainApiResponseBuilder::created_at)
    /// - [`domain_name`](PaymentMethodDomainApiResponseBuilder::domain_name)
    /// - [`entity_id`](PaymentMethodDomainApiResponseBuilder::entity_id)
    /// - [`entity_type`](PaymentMethodDomainApiResponseBuilder::entity_type)
    /// - [`id`](PaymentMethodDomainApiResponseBuilder::id)
    /// - [`owner_entity_id`](PaymentMethodDomainApiResponseBuilder::owner_entity_id)
    /// - [`owner_entity_type`](PaymentMethodDomainApiResponseBuilder::owner_entity_type)
    pub fn build(self) -> Result<PaymentMethodDomainApiResponse, BuildError> {
        Ok(PaymentMethodDomainApiResponse {
            r#type: self.r#type,
            apple_pay: self
                .apple_pay
                .ok_or_else(|| BuildError::missing_field("apple_pay"))?,
            google_pay: self
                .google_pay
                .ok_or_else(|| BuildError::missing_field("google_pay"))?,
            cascades: self.cascades,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            owner_entity_id: self
                .owner_entity_id
                .ok_or_else(|| BuildError::missing_field("owner_entity_id"))?,
            owner_entity_type: self
                .owner_entity_type
                .ok_or_else(|| BuildError::missing_field("owner_entity_type"))?,
            updated_at: self.updated_at,
        })
    }
}
