pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentMethodRequest {
    /// An optional label for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The bank or financial institution that holds the account.
    #[serde(rename = "financialInstitution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_institution: Option<String>,
    /// The account type, for example `checking` or `savings`.
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
    /// What the payment method is used for. Provide at least one value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<PaymentMethodUsage>>,
}

impl UpdatePaymentMethodRequest {
    pub fn builder() -> UpdatePaymentMethodRequestBuilder {
        <UpdatePaymentMethodRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentMethodRequestBuilder {
    nickname: Option<String>,
    financial_institution: Option<String>,
    account_type: Option<String>,
    account_number: Option<String>,
    routing_number: Option<String>,
    usage: Option<Vec<PaymentMethodUsage>>,
}

impl UpdatePaymentMethodRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePaymentMethodRequest`].
    pub fn build(self) -> Result<UpdatePaymentMethodRequest, BuildError> {
        Ok(UpdatePaymentMethodRequest {
            nickname: self.nickname,
            financial_institution: self.financial_institution,
            account_type: self.account_type,
            account_number: self.account_number,
            routing_number: self.routing_number,
            usage: self.usage,
        })
    }
}
