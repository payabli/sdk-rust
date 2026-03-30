pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillingDataResponse {
    /// The bank's ID in Payabli.
    #[serde(default)]
    pub id: i64,
    /// An identifier for the bank account. If not provided during creation or update, the system generates one in the format `acct-{first_digit}xxxxx{last_4_digits}` based on the account number. If a duplicate exists within the same service at the paypoint, a numeric suffix is appended, such as `-2`. This value is also used as the identifier for the bank account's associated payment connector.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(default)]
    pub nickname: String,
    #[serde(rename = "bankName")]
    #[serde(default)]
    pub bank_name: BankName,
    #[serde(rename = "routingAccount")]
    #[serde(default)]
    pub routing_account: RoutingAccount,
    #[serde(rename = "accountNumber")]
    #[serde(default)]
    pub account_number: AccountNumber,
    #[serde(rename = "typeAccount")]
    pub type_account: TypeAccount,
    #[serde(rename = "bankAccountHolderName")]
    #[serde(default)]
    pub bank_account_holder_name: BankAccountHolderName,
    #[serde(rename = "bankAccountHolderType")]
    pub bank_account_holder_type: BankAccountHolderType,
    /// Describes whether the bank account is used for deposits or withdrawals in Payabli:
    /// - `0`: Deposit
    /// - `1`: Withdrawal
    /// - `2`: Deposit and withdrawal
    #[serde(rename = "bankAccountFunction")]
    #[serde(default)]
    pub bank_account_function: i64,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub status: i64,
    #[serde(default)]
    pub services: Vec<serde_json::Value>,
    #[serde(default)]
    pub default: bool,
}

impl BillingDataResponse {
    pub fn builder() -> BillingDataResponseBuilder {
        <BillingDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingDataResponseBuilder {
    id: Option<i64>,
    account_id: Option<AccountId>,
    nickname: Option<String>,
    bank_name: Option<BankName>,
    routing_account: Option<RoutingAccount>,
    account_number: Option<AccountNumber>,
    type_account: Option<TypeAccount>,
    bank_account_holder_name: Option<BankAccountHolderName>,
    bank_account_holder_type: Option<BankAccountHolderType>,
    bank_account_function: Option<i64>,
    verified: Option<bool>,
    status: Option<i64>,
    services: Option<Vec<serde_json::Value>>,
    default: Option<bool>,
}

impl BillingDataResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
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

    /// Consumes the builder and constructs a [`BillingDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BillingDataResponseBuilder::id)
    /// - [`nickname`](BillingDataResponseBuilder::nickname)
    /// - [`bank_name`](BillingDataResponseBuilder::bank_name)
    /// - [`routing_account`](BillingDataResponseBuilder::routing_account)
    /// - [`account_number`](BillingDataResponseBuilder::account_number)
    /// - [`type_account`](BillingDataResponseBuilder::type_account)
    /// - [`bank_account_holder_name`](BillingDataResponseBuilder::bank_account_holder_name)
    /// - [`bank_account_holder_type`](BillingDataResponseBuilder::bank_account_holder_type)
    /// - [`bank_account_function`](BillingDataResponseBuilder::bank_account_function)
    /// - [`verified`](BillingDataResponseBuilder::verified)
    /// - [`status`](BillingDataResponseBuilder::status)
    /// - [`services`](BillingDataResponseBuilder::services)
    /// - [`default`](BillingDataResponseBuilder::default)
    pub fn build(self) -> Result<BillingDataResponse, BuildError> {
        Ok(BillingDataResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            account_id: self.account_id,
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
            routing_account: self
                .routing_account
                .ok_or_else(|| BuildError::missing_field("routing_account"))?,
            account_number: self
                .account_number
                .ok_or_else(|| BuildError::missing_field("account_number"))?,
            type_account: self
                .type_account
                .ok_or_else(|| BuildError::missing_field("type_account"))?,
            bank_account_holder_name: self
                .bank_account_holder_name
                .ok_or_else(|| BuildError::missing_field("bank_account_holder_name"))?,
            bank_account_holder_type: self
                .bank_account_holder_type
                .ok_or_else(|| BuildError::missing_field("bank_account_holder_type"))?,
            bank_account_function: self
                .bank_account_function
                .ok_or_else(|| BuildError::missing_field("bank_account_function"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
            default: self
                .default
                .ok_or_else(|| BuildError::missing_field("default"))?,
        })
    }
}
