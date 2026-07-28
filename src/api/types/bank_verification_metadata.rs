pub use crate::prelude::*;

/// The outcome of automatic bank account verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankVerificationMetadata {
    #[serde(rename = "verificationResult")]
    #[serde(default)]
    pub verification_result: VerificationCode,
    /// The account-level verification code. Null when not returned.
    #[serde(rename = "accountResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_response_code: Option<VerificationCode>,
    /// The customer-level verification code. Null when not returned.
    #[serde(rename = "customerResponseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_response_code: Option<VerificationCode>,
}

impl BankVerificationMetadata {
    pub fn builder() -> BankVerificationMetadataBuilder {
        <BankVerificationMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankVerificationMetadataBuilder {
    verification_result: Option<VerificationCode>,
    account_response_code: Option<VerificationCode>,
    customer_response_code: Option<VerificationCode>,
}

impl BankVerificationMetadataBuilder {
    pub fn verification_result(mut self, value: VerificationCode) -> Self {
        self.verification_result = Some(value);
        self
    }

    pub fn account_response_code(mut self, value: VerificationCode) -> Self {
        self.account_response_code = Some(value);
        self
    }

    pub fn customer_response_code(mut self, value: VerificationCode) -> Self {
        self.customer_response_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankVerificationMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verification_result`](BankVerificationMetadataBuilder::verification_result)
    pub fn build(self) -> Result<BankVerificationMetadata, BuildError> {
        Ok(BankVerificationMetadata {
            verification_result: self
                .verification_result
                .ok_or_else(|| BuildError::missing_field("verification_result"))?,
            account_response_code: self.account_response_code,
            customer_response_code: self.customer_response_code,
        })
    }
}
