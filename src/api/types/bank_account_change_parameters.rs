pub use crate::prelude::*;

/// The bank-account-change details stored on a case. The raw account and
/// routing numbers are write-only and never appear here — only a vault token
/// (`bankToken`) and non-sensitive details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BankAccountChangeParameters {
    /// The parameters type discriminator.
    pub r#type: BankAccountChangeParametersType,
    /// A label for the account.
    #[serde(default)]
    pub nickname: String,
    /// The name of the bank.
    #[serde(rename = "bankName")]
    #[serde(default)]
    pub bank_name: String,
    /// A vault token referencing the tokenized bank account. The raw account and routing numbers are never returned.
    #[serde(rename = "bankToken")]
    #[serde(default)]
    pub bank_token: String,
    /// The account type, such as `Checking` or `Savings`.
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: String,
    /// The account holder's name, taken from the paypoint's legal name.
    #[serde(rename = "bankAccountHolderName")]
    #[serde(default)]
    pub bank_account_holder_name: String,
    /// The account holder type, such as `personal` or `business`.
    #[serde(rename = "bankAccountHolderType")]
    #[serde(default)]
    pub bank_account_holder_type: String,
    #[serde(rename = "bankAccountFunction")]
    pub bank_account_function: CaseManagementBankAccountFunction,
    #[serde(default)]
    pub services: BankAccountServices,
    /// Whether this is the default account for the selected services.
    #[serde(default)]
    pub default: bool,
}

impl BankAccountChangeParameters {
    pub fn builder() -> BankAccountChangeParametersBuilder {
        <BankAccountChangeParametersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountChangeParametersBuilder {
    r#type: Option<BankAccountChangeParametersType>,
    nickname: Option<String>,
    bank_name: Option<String>,
    bank_token: Option<String>,
    account_type: Option<String>,
    bank_account_holder_name: Option<String>,
    bank_account_holder_type: Option<String>,
    bank_account_function: Option<CaseManagementBankAccountFunction>,
    services: Option<BankAccountServices>,
    default: Option<bool>,
}

impl BankAccountChangeParametersBuilder {
    pub fn r#type(mut self, value: BankAccountChangeParametersType) -> Self {
        self.r#type = Some(value);
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

    pub fn bank_token(mut self, value: impl Into<String>) -> Self {
        self.bank_token = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
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

    pub fn bank_account_function(mut self, value: CaseManagementBankAccountFunction) -> Self {
        self.bank_account_function = Some(value);
        self
    }

    pub fn services(mut self, value: BankAccountServices) -> Self {
        self.services = Some(value);
        self
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountChangeParameters`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](BankAccountChangeParametersBuilder::r#type)
    /// - [`nickname`](BankAccountChangeParametersBuilder::nickname)
    /// - [`bank_name`](BankAccountChangeParametersBuilder::bank_name)
    /// - [`bank_token`](BankAccountChangeParametersBuilder::bank_token)
    /// - [`account_type`](BankAccountChangeParametersBuilder::account_type)
    /// - [`bank_account_holder_name`](BankAccountChangeParametersBuilder::bank_account_holder_name)
    /// - [`bank_account_holder_type`](BankAccountChangeParametersBuilder::bank_account_holder_type)
    /// - [`bank_account_function`](BankAccountChangeParametersBuilder::bank_account_function)
    /// - [`services`](BankAccountChangeParametersBuilder::services)
    /// - [`default`](BankAccountChangeParametersBuilder::default)
    pub fn build(self) -> Result<BankAccountChangeParameters, BuildError> {
        Ok(BankAccountChangeParameters {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
            bank_token: self
                .bank_token
                .ok_or_else(|| BuildError::missing_field("bank_token"))?,
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
            bank_account_holder_name: self
                .bank_account_holder_name
                .ok_or_else(|| BuildError::missing_field("bank_account_holder_name"))?,
            bank_account_holder_type: self
                .bank_account_holder_type
                .ok_or_else(|| BuildError::missing_field("bank_account_holder_type"))?,
            bank_account_function: self
                .bank_account_function
                .ok_or_else(|| BuildError::missing_field("bank_account_function"))?,
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
            default: self
                .default
                .ok_or_else(|| BuildError::missing_field("default"))?,
        })
    }
}
