pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingData {
    /// Account number for bank account.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Describes whether the bank account is used for deposits or withdrawals in Payabli:
    /// - `0`: Deposit
    /// - `1`: Withdrawal
    /// - `2`: Deposit and withdrawal
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<BankAccountHolderName>,
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<BankAccountHolderType>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<BankName>,
    /// The bank's ID in Payabli.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<RoutingAccount>,
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<TypeAccount>,
}

impl BillingData {
    pub fn builder() -> BillingDataBuilder {
        <BillingDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingDataBuilder {
    account_number: Option<String>,
    bank_account_function: Option<i64>,
    bank_account_holder_name: Option<BankAccountHolderName>,
    bank_account_holder_type: Option<BankAccountHolderType>,
    bank_name: Option<BankName>,
    id: Option<i64>,
    routing_account: Option<RoutingAccount>,
    type_account: Option<TypeAccount>,
}

impl BillingDataBuilder {
    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn bank_account_function(mut self, value: i64) -> Self {
        self.bank_account_function = Some(value);
        self
    }

    pub fn bank_account_holder_name(mut self, value: BankAccountHolderName) -> Self {
        self.bank_account_holder_name = Some(value);
        self
    }

    pub fn bank_account_holder_type(mut self, value: BankAccountHolderType) -> Self {
        self.bank_account_holder_type = Some(value);
        self
    }

    pub fn bank_name(mut self, value: BankName) -> Self {
        self.bank_name = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn routing_account(mut self, value: RoutingAccount) -> Self {
        self.routing_account = Some(value);
        self
    }

    pub fn type_account(mut self, value: TypeAccount) -> Self {
        self.type_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingData`].
    pub fn build(self) -> Result<BillingData, BuildError> {
        Ok(BillingData {
            account_number: self.account_number,
            bank_account_function: self.bank_account_function,
            bank_account_holder_name: self.bank_account_holder_name,
            bank_account_holder_type: self.bank_account_holder_type,
            bank_name: self.bank_name,
            id: self.id,
            routing_account: self.routing_account,
            type_account: self.type_account,
        })
    }
}
