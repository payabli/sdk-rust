pub use crate::prelude::*;

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

impl AddPaymentMethodDomainRequest {
    pub fn builder() -> AddPaymentMethodDomainRequestBuilder {
        <AddPaymentMethodDomainRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPaymentMethodDomainRequestBuilder {
    apple_pay: Option<AddPaymentMethodDomainRequestApplePay>,
    google_pay: Option<AddPaymentMethodDomainRequestGooglePay>,
    domain_name: Option<DomainName>,
    entity_id: Option<EntityId>,
    entity_type: Option<EntityType>,
}

impl AddPaymentMethodDomainRequestBuilder {
    pub fn apple_pay(mut self, value: AddPaymentMethodDomainRequestApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn google_pay(mut self, value: AddPaymentMethodDomainRequestGooglePay) -> Self {
        self.google_pay = Some(value);
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

    /// Consumes the builder and constructs a [`AddPaymentMethodDomainRequest`].
    pub fn build(self) -> Result<AddPaymentMethodDomainRequest, BuildError> {
        Ok(AddPaymentMethodDomainRequest {
            apple_pay: self.apple_pay,
            google_pay: self.google_pay,
            domain_name: self.domain_name,
            entity_id: self.entity_id,
            entity_type: self.entity_type,
        })
    }
}
