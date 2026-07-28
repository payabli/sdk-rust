pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidateBankAccountChangeRequest {
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
}

impl ValidateBankAccountChangeRequest {
    pub fn builder() -> ValidateBankAccountChangeRequestBuilder {
        <ValidateBankAccountChangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidateBankAccountChangeRequestBuilder {
    routing_number: Option<String>,
    account_number: Option<String>,
    account_type: Option<String>,
    bank_account_holder_type: Option<String>,
    bank_account_function: Option<CaseManagementBankAccountFunction>,
    services: Option<BankAccountServices>,
}

impl ValidateBankAccountChangeRequestBuilder {
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

    /// Consumes the builder and constructs a [`ValidateBankAccountChangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`routing_number`](ValidateBankAccountChangeRequestBuilder::routing_number)
    /// - [`account_number`](ValidateBankAccountChangeRequestBuilder::account_number)
    /// - [`account_type`](ValidateBankAccountChangeRequestBuilder::account_type)
    /// - [`bank_account_holder_type`](ValidateBankAccountChangeRequestBuilder::bank_account_holder_type)
    /// - [`bank_account_function`](ValidateBankAccountChangeRequestBuilder::bank_account_function)
    /// - [`services`](ValidateBankAccountChangeRequestBuilder::services)
    pub fn build(self) -> Result<ValidateBankAccountChangeRequest, BuildError> {
        Ok(ValidateBankAccountChangeRequest {
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
        })
    }
}
