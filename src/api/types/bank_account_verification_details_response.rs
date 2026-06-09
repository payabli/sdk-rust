pub use crate::prelude::*;

/// Detailed bank account verification results from the verification network.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountVerificationDetailsResponse {
    /// The ABA routing number that was verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<String>,
    /// The account number that was verified.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Whether the bank account passed verification.
    #[serde(rename = "isValid")]
    #[serde(default)]
    pub is_valid: bool,
    /// Error message if the verification request failed.
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Overall verification outcome. Possible values include `Pass`, `Verified`, `Declined`, `NoData`, `Bypassed`, and `Error`.
    #[serde(rename = "verificationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_response: Option<String>,
    /// Response code returned by the verification network.
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// Response value associated with the verification outcome.
    #[serde(rename = "responseValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_value: Option<String>,
    /// Human-readable description of the verification outcome.
    #[serde(rename = "responseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_description: Option<String>,
    /// Name of the bank associated with the routing number.
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// Account type as reported by the verification network, such as `Checking` or `Savings`.
    #[serde(rename = "reportedAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_account_type: Option<String>,
    /// Date the account was first seen by the verification network (ISO 8601 format).
    #[serde(rename = "accountAddedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_added_date: Option<String>,
    /// Date the account record was last updated in the verification network (ISO 8601 format).
    #[serde(rename = "accountLastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_last_updated_date: Option<String>,
    /// Date the account was closed, if applicable (ISO 8601 format).
    #[serde(rename = "accountClosedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_closed_date: Option<String>,
}

impl BankAccountVerificationDetailsResponse {
    pub fn builder() -> BankAccountVerificationDetailsResponseBuilder {
        <BankAccountVerificationDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountVerificationDetailsResponseBuilder {
    aba: Option<String>,
    account_number: Option<String>,
    is_valid: Option<bool>,
    error_message: Option<String>,
    verification_response: Option<String>,
    response_code: Option<String>,
    response_value: Option<String>,
    response_description: Option<String>,
    bank_name: Option<String>,
    reported_account_type: Option<String>,
    account_added_date: Option<String>,
    account_last_updated_date: Option<String>,
    account_closed_date: Option<String>,
}

impl BankAccountVerificationDetailsResponseBuilder {
    pub fn aba(mut self, value: impl Into<String>) -> Self {
        self.aba = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn is_valid(mut self, value: bool) -> Self {
        self.is_valid = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn verification_response(mut self, value: impl Into<String>) -> Self {
        self.verification_response = Some(value.into());
        self
    }

    pub fn response_code(mut self, value: impl Into<String>) -> Self {
        self.response_code = Some(value.into());
        self
    }

    pub fn response_value(mut self, value: impl Into<String>) -> Self {
        self.response_value = Some(value.into());
        self
    }

    pub fn response_description(mut self, value: impl Into<String>) -> Self {
        self.response_description = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn reported_account_type(mut self, value: impl Into<String>) -> Self {
        self.reported_account_type = Some(value.into());
        self
    }

    pub fn account_added_date(mut self, value: impl Into<String>) -> Self {
        self.account_added_date = Some(value.into());
        self
    }

    pub fn account_last_updated_date(mut self, value: impl Into<String>) -> Self {
        self.account_last_updated_date = Some(value.into());
        self
    }

    pub fn account_closed_date(mut self, value: impl Into<String>) -> Self {
        self.account_closed_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BankAccountVerificationDetailsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_valid`](BankAccountVerificationDetailsResponseBuilder::is_valid)
    pub fn build(self) -> Result<BankAccountVerificationDetailsResponse, BuildError> {
        Ok(BankAccountVerificationDetailsResponse {
            aba: self.aba,
            account_number: self.account_number,
            is_valid: self
                .is_valid
                .ok_or_else(|| BuildError::missing_field("is_valid"))?,
            error_message: self.error_message,
            verification_response: self.verification_response,
            response_code: self.response_code,
            response_value: self.response_value,
            response_description: self.response_description,
            bank_name: self.bank_name,
            reported_account_type: self.reported_account_type,
            account_added_date: self.account_added_date,
            account_last_updated_date: self.account_last_updated_date,
            account_closed_date: self.account_closed_date,
        })
    }
}
