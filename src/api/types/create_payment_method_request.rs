pub use crate::prelude::*;

/// A single payment method to create.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentMethodRequest {
    /// The payment method type.
    pub r#type: CreatePaymentMethodRequestType,
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
    /// The account type, for example `checking` or `savings`.
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: String,
    /// The bank account number.
    #[serde(rename = "accountNumber")]
    #[serde(default)]
    pub account_number: String,
    /// The bank routing number.
    #[serde(rename = "routingNumber")]
    #[serde(default)]
    pub routing_number: String,
    /// What the payment method is used for. Provide at least one value.
    #[serde(default)]
    pub usage: Vec<PaymentMethodUsage>,
    /// When `true`, makes this the owner's default payment method. If the owner
    /// already has a default, this payment method replaces it.
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl CreatePaymentMethodRequest {
    pub fn builder() -> CreatePaymentMethodRequestBuilder {
        <CreatePaymentMethodRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentMethodRequestBuilder {
    r#type: Option<CreatePaymentMethodRequestType>,
    account_owner: Option<String>,
    owner_type: Option<PaymentMethodOwnerType>,
    nickname: Option<String>,
    financial_institution: Option<String>,
    account_type: Option<String>,
    account_number: Option<String>,
    routing_number: Option<String>,
    usage: Option<Vec<PaymentMethodUsage>>,
    is_default: Option<bool>,
}

impl CreatePaymentMethodRequestBuilder {
    pub fn r#type(mut self, value: CreatePaymentMethodRequestType) -> Self {
        self.r#type = Some(value);
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

    pub fn usage(mut self, value: Vec<PaymentMethodUsage>) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentMethodRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreatePaymentMethodRequestBuilder::r#type)
    /// - [`account_owner`](CreatePaymentMethodRequestBuilder::account_owner)
    /// - [`owner_type`](CreatePaymentMethodRequestBuilder::owner_type)
    /// - [`account_type`](CreatePaymentMethodRequestBuilder::account_type)
    /// - [`account_number`](CreatePaymentMethodRequestBuilder::account_number)
    /// - [`routing_number`](CreatePaymentMethodRequestBuilder::routing_number)
    /// - [`usage`](CreatePaymentMethodRequestBuilder::usage)
    pub fn build(self) -> Result<CreatePaymentMethodRequest, BuildError> {
        Ok(CreatePaymentMethodRequest {
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
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
            account_number: self
                .account_number
                .ok_or_else(|| BuildError::missing_field("account_number"))?,
            routing_number: self
                .routing_number
                .ok_or_else(|| BuildError::missing_field("routing_number"))?,
            usage: self
                .usage
                .ok_or_else(|| BuildError::missing_field("usage"))?,
            is_default: self.is_default,
        })
    }
}
