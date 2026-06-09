pub use crate::prelude::*;

/// Object that contains bank account details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Bank {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The Payabli-assigned internal identifier for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// An identifier for the bank account, used to specify which account handles payments when multiple accounts are configured. If not provided during creation or update, the system generates one in the format `acct-{first_digit}xxxxx{last_4_digits}` based on the account number. The mask always uses five `x` characters regardless of account number length. For example, account number `123456789` produces `acct-1xxxxx6789`. If a duplicate exists within the same service at the paypoint, a numeric suffix is appended, such as `acct-1xxxxx6789-2`. This value is also used as the identifier for the bank account's associated payment connector.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<BankNickname>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<BankName>,
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<RoutingAccount>,
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<AccountNumber>,
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<TypeAccount>,
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<BankAccountHolderName>,
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<BankAccountHolderType>,
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<BankAccountFunction>,
    /// Bank account verification status. When `true`, the account has been verified to exist and be in good standing based on vendor checks or previous processing histories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Bank account status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Array of services associated with this bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl Bank {
    pub fn builder() -> BankBuilder {
        <BankBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankBuilder {
    default: Option<bool>,
    country: Option<String>,
    id: Option<i64>,
    account_id: Option<AccountId>,
    nickname: Option<BankNickname>,
    bank_name: Option<BankName>,
    routing_account: Option<RoutingAccount>,
    account_number: Option<AccountNumber>,
    type_account: Option<TypeAccount>,
    bank_account_holder_name: Option<BankAccountHolderName>,
    bank_account_holder_type: Option<BankAccountHolderType>,
    bank_account_function: Option<BankAccountFunction>,
    verified: Option<bool>,
    status: Option<i64>,
    services: Option<Vec<String>>,
}

impl BankBuilder {
    pub fn default(mut self, value: bool) -> Self {
        self.default = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn nickname(mut self, value: BankNickname) -> Self {
        self.nickname = Some(value);
        self
    }

    pub fn bank_name(mut self, value: BankName) -> Self {
        self.bank_name = Some(value);
        self
    }

    pub fn routing_account(mut self, value: RoutingAccount) -> Self {
        self.routing_account = Some(value);
        self
    }

    pub fn account_number(mut self, value: AccountNumber) -> Self {
        self.account_number = Some(value);
        self
    }

    pub fn type_account(mut self, value: TypeAccount) -> Self {
        self.type_account = Some(value);
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

    pub fn bank_account_function(mut self, value: BankAccountFunction) -> Self {
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

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bank`].
    pub fn build(self) -> Result<Bank, BuildError> {
        Ok(Bank {
            default: self.default,
            country: self.country,
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
        })
    }
}
