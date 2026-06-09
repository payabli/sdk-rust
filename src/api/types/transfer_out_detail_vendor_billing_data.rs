pub use crate::prelude::*;

/// Billing data for a vendor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutDetailVendorBillingData {
    /// Unique identifier for the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The account ID.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A nickname for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The name of the bank.
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The routing number.
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<String>,
    /// The account number.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The type of account.
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<String>,
    /// The name of the account holder.
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<String>,
    /// The type of account holder.
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<String>,
    /// The function of the bank account.
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
    /// Whether the account is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The status of the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Services associated with the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<serde_json::Value>>,
    /// Whether this is the default billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The country of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl TransferOutDetailVendorBillingData {
    pub fn builder() -> TransferOutDetailVendorBillingDataBuilder {
        <TransferOutDetailVendorBillingDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailVendorBillingDataBuilder {
    id: Option<i64>,
    account_id: Option<String>,
    nickname: Option<String>,
    bank_name: Option<String>,
    routing_account: Option<String>,
    account_number: Option<String>,
    type_account: Option<String>,
    bank_account_holder_name: Option<String>,
    bank_account_holder_type: Option<String>,
    bank_account_function: Option<i64>,
    verified: Option<bool>,
    status: Option<i64>,
    services: Option<Vec<serde_json::Value>>,
    default: Option<bool>,
    country: Option<String>,
}

impl TransferOutDetailVendorBillingDataBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn routing_account(mut self, value: impl Into<String>) -> Self {
        self.routing_account = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn type_account(mut self, value: impl Into<String>) -> Self {
        self.type_account = Some(value.into());
        self
    }

    pub fn bank_account_holder_name(mut self, value: impl Into<String>) -> Self {
        self.bank_account_holder_name = Some(value.into());
        self
    }

    pub fn bank_account_holder_type(mut self, value: impl Into<String>) -> Self {
        self.bank_account_holder_type = Some(value.into());
        self
    }

    pub fn bank_account_function(mut self, value: i64) -> Self {
        self.bank_account_function = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<serde_json::Value>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailVendorBillingData`].
    pub fn build(self) -> Result<TransferOutDetailVendorBillingData, BuildError> {
        Ok(TransferOutDetailVendorBillingData {
            id: self.id,
            account_id: self.account_id,
            nickname: self.nickname,
            bank_name: self.bank_name,
            routing_account: self.routing_account,
            account_number: self.account_number,
            type_account: self.type_account,
            bank_account_holder_name: self.bank_account_holder_name,
            bank_account_holder_type: self.bank_account_holder_type,
            bank_account_function: self.bank_account_function,
            verified: self.verified,
            status: self.status,
            services: self.services,
            default: self.default,
            country: self.country,
        })
    }
}
