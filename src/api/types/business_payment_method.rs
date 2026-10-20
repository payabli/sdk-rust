pub use crate::prelude::*;

/// A bank account or card attached to a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessPaymentMethod {
    /// The payment method type, for example a bank account or card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The name of the account owner.
    #[serde(rename = "accountOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    /// An optional label for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The bank or financial institution that holds the account.
    #[serde(rename = "financialInstitution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution: Option<String>,
    /// Whether the owner is a business or an individual.
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    /// The account type, for example checking or savings.
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// The bank account number.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The bank routing number.
    #[serde(rename = "routingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /// The card number, for card payment methods.
    #[serde(rename = "cardNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// The card expiration date, for card payment methods.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// The card verification value, for card payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv: Option<String>,
    /// The card type, for card payment methods.
    #[serde(rename = "cardType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// What the payment method is used for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<String>>,
    /// Whether this is the owner's default payment method. At most one payment
    /// method per owner can set `isDefault` to `true`; sending two `true` values
    /// in one request fails validation.
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl BusinessPaymentMethod {
    pub fn builder() -> BusinessPaymentMethodBuilder {
        <BusinessPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessPaymentMethodBuilder {
    r#type: Option<String>,
    account_owner: Option<String>,
    name: Option<String>,
    financial_institution: Option<String>,
    owner_type: Option<String>,
    account_type: Option<String>,
    account_number: Option<String>,
    routing_number: Option<String>,
    card_number: Option<String>,
    expiration_date: Option<String>,
    cvv: Option<String>,
    card_type: Option<String>,
    usage: Option<Vec<String>>,
    is_default: Option<bool>,
}

impl BusinessPaymentMethodBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn account_owner(mut self, value: impl Into<String>) -> Self {
        self.account_owner = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn financial_institution(mut self, value: impl Into<String>) -> Self {
        self.financial_institution = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: impl Into<String>) -> Self {
        self.owner_type = Some(value.into());
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

    pub fn card_number(mut self, value: impl Into<String>) -> Self {
        self.card_number = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn cvv(mut self, value: impl Into<String>) -> Self {
        self.cvv = Some(value.into());
        self
    }

    pub fn card_type(mut self, value: impl Into<String>) -> Self {
        self.card_type = Some(value.into());
        self
    }

    pub fn usage(mut self, value: Vec<String>) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessPaymentMethod`].
    pub fn build(self) -> Result<BusinessPaymentMethod, BuildError> {
        Ok(BusinessPaymentMethod {
            r#type: self.r#type,
            account_owner: self.account_owner,
            name: self.name,
            financial_institution: self.financial_institution,
            owner_type: self.owner_type,
            account_type: self.account_type,
            account_number: self.account_number,
            routing_number: self.routing_number,
            card_number: self.card_number,
            expiration_date: self.expiration_date,
            cvv: self.cvv,
            card_type: self.card_type,
            usage: self.usage,
            is_default: self.is_default,
        })
    }
}
