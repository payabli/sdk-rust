pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBankAccountChangeCaseRequest {
    /// A label for the account.
    #[serde(default)]
    pub nickname: String,
    /// The name of the bank.
    #[serde(rename = "bankName")]
    #[serde(default)]
    pub bank_name: String,
    /// The 9-digit bank routing number.
    #[serde(rename = "routingNumber")]
    #[serde(default)]
    pub routing_number: String,
    /// The bank account number (4 to 17 digits).
    #[serde(rename = "accountNumber")]
    #[serde(default)]
    pub account_number: String,
    /// The account type. Must be `checking` or `savings`.
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: String,
    /// The account holder type. Must be `personal` or `business`.
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
    /// When to run the change, as a UTC timestamp (trailing `Z`). Must be at
    /// least 1 hour and at most 30 days in the future. Omit to run as soon as
    /// the case is approved.
    #[serde(rename = "scheduleFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub schedule_for: Option<DateTime<Utc>>,
}

impl CreateBankAccountChangeCaseRequest {
    pub fn builder() -> CreateBankAccountChangeCaseRequestBuilder {
        <CreateBankAccountChangeCaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBankAccountChangeCaseRequestBuilder {
    nickname: Option<String>,
    bank_name: Option<String>,
    routing_number: Option<String>,
    account_number: Option<String>,
    account_type: Option<String>,
    bank_account_holder_type: Option<String>,
    bank_account_function: Option<CaseManagementBankAccountFunction>,
    services: Option<BankAccountServices>,
    default: Option<bool>,
    schedule_for: Option<DateTime<Utc>>,
}

impl CreateBankAccountChangeCaseRequestBuilder {
    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn routing_number(mut self, value: impl Into<String>) -> Self {
        self.routing_number = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
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

    pub fn schedule_for(mut self, value: DateTime<Utc>) -> Self {
        self.schedule_for = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBankAccountChangeCaseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`nickname`](CreateBankAccountChangeCaseRequestBuilder::nickname)
    /// - [`bank_name`](CreateBankAccountChangeCaseRequestBuilder::bank_name)
    /// - [`routing_number`](CreateBankAccountChangeCaseRequestBuilder::routing_number)
    /// - [`account_number`](CreateBankAccountChangeCaseRequestBuilder::account_number)
    /// - [`account_type`](CreateBankAccountChangeCaseRequestBuilder::account_type)
    /// - [`bank_account_holder_type`](CreateBankAccountChangeCaseRequestBuilder::bank_account_holder_type)
    /// - [`bank_account_function`](CreateBankAccountChangeCaseRequestBuilder::bank_account_function)
    /// - [`services`](CreateBankAccountChangeCaseRequestBuilder::services)
    /// - [`default`](CreateBankAccountChangeCaseRequestBuilder::default)
    pub fn build(self) -> Result<CreateBankAccountChangeCaseRequest, BuildError> {
        Ok(CreateBankAccountChangeCaseRequest {
            nickname: self
                .nickname
                .ok_or_else(|| BuildError::missing_field("nickname"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
            routing_number: self
                .routing_number
                .ok_or_else(|| BuildError::missing_field("routing_number"))?,
            account_number: self
                .account_number
                .ok_or_else(|| BuildError::missing_field("account_number"))?,
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
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
            schedule_for: self.schedule_for,
        })
    }
}
