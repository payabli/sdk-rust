pub use crate::prelude::*;

/// A payment method as returned by read and list. Sensitive fields are masked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMethodDetail {
    /// Whether this is the owner's default payment method. Returned when you list payment methods, but not when you get a single payment method.
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The payment method reference.
    #[serde(rename = "paymentMethodReference")]
    #[serde(default)]
    pub payment_method_reference: String,
    /// The payment method type.
    #[serde(default)]
    pub r#type: String,
    /// The reference of the person or business that owns the payment method (for example, a business `businessUuid` or a person `personReference`).
    #[serde(rename = "accountOwner")]
    #[serde(default)]
    pub account_owner: String,
    /// Whether the owner is a `person` or a `business`.
    #[serde(rename = "ownerType")]
    pub owner_type: PaymentMethodOwnerType,
    /// An optional label for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The bank or financial institution that holds the account.
    #[serde(rename = "financialInstitution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution: Option<String>,
    /// The account type.
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// The bank account number, masked (only the last four digits are shown).
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The bank routing number, masked.
    #[serde(rename = "routingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /// The payment method's current status.
    #[serde(default)]
    pub status: String,
    /// What the payment method is used for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<PaymentMethodUsage>>,
    /// When the payment method was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// When the payment method was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    pub last_updated: String,
    /// Hypermedia links for the payment method resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PaymentMethodLinks,
}

impl PaymentMethodDetail {
    pub fn builder() -> PaymentMethodDetailBuilder {
        <PaymentMethodDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDetailBuilder {
    is_default: Option<bool>,
    payment_method_reference: Option<String>,
    r#type: Option<String>,
    account_owner: Option<String>,
    owner_type: Option<PaymentMethodOwnerType>,
    nickname: Option<String>,
    financial_institution: Option<String>,
    account_type: Option<String>,
    account_number: Option<String>,
    routing_number: Option<String>,
    status: Option<String>,
    usage: Option<Vec<PaymentMethodUsage>>,
    created_at: Option<String>,
    last_updated: Option<String>,
    links: Option<PaymentMethodLinks>,
}

impl PaymentMethodDetailBuilder {
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn payment_method_reference(mut self, value: impl Into<String>) -> Self {
        self.payment_method_reference = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn account_owner(mut self, value: impl Into<String>) -> Self {
        self.account_owner = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: PaymentMethodOwnerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn financial_institution(mut self, value: impl Into<String>) -> Self {
        self.financial_institution = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn routing_number(mut self, value: impl Into<String>) -> Self {
        self.routing_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn usage(mut self, value: Vec<PaymentMethodUsage>) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: impl Into<String>) -> Self {
        self.last_updated = Some(value.into());
        self
    }

    pub fn links(mut self, value: PaymentMethodLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_method_reference`](PaymentMethodDetailBuilder::payment_method_reference)
    /// - [`r#type`](PaymentMethodDetailBuilder::r#type)
    /// - [`account_owner`](PaymentMethodDetailBuilder::account_owner)
    /// - [`owner_type`](PaymentMethodDetailBuilder::owner_type)
    /// - [`status`](PaymentMethodDetailBuilder::status)
    /// - [`created_at`](PaymentMethodDetailBuilder::created_at)
    /// - [`last_updated`](PaymentMethodDetailBuilder::last_updated)
    /// - [`links`](PaymentMethodDetailBuilder::links)
    pub fn build(self) -> Result<PaymentMethodDetail, BuildError> {
        Ok(PaymentMethodDetail {
            is_default: self.is_default,
            payment_method_reference: self
                .payment_method_reference
                .ok_or_else(|| BuildError::missing_field("payment_method_reference"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            account_owner: self
                .account_owner
                .ok_or_else(|| BuildError::missing_field("account_owner"))?,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            nickname: self.nickname,
            financial_institution: self.financial_institution,
            account_type: self.account_type,
            account_number: self.account_number,
            routing_number: self.routing_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            usage: self.usage,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
