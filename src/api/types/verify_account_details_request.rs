pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerifyAccountDetailsRequest {
    /// The bank routing number to verify.
    #[serde(rename = "routingNumber")]
    #[serde(default)]
    pub routing_number: String,
    /// The bank account number to verify.
    #[serde(rename = "accountNumber")]
    #[serde(default)]
    pub account_number: String,
    /// The type of bank account, such as `Checking` or `Savings`.
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// The ISO country code for the bank account, such as `US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The type of account holder. Accepted values are `personal` or `business`. Required when bank authentication is enabled for the paypoint's organization.
    #[serde(rename = "accountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,
    /// The name of the bank account holder. For personal accounts, provide the holder's full name (for example, `Jane Doe`); the value is split on the first space into first and last name. For business accounts, provide the legal business name. Required when bank authentication is enabled for the paypoint's organization.
    #[serde(rename = "holderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<String>,
}

impl VerifyAccountDetailsRequest {
    pub fn builder() -> VerifyAccountDetailsRequestBuilder {
        <VerifyAccountDetailsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifyAccountDetailsRequestBuilder {
    routing_number: Option<String>,
    account_number: Option<String>,
    account_type: Option<String>,
    country: Option<String>,
    account_holder_type: Option<String>,
    holder_name: Option<String>,
}

impl VerifyAccountDetailsRequestBuilder {
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

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn account_holder_type(mut self, value: impl Into<String>) -> Self {
        self.account_holder_type = Some(value.into());
        self
    }

    pub fn holder_name(mut self, value: impl Into<String>) -> Self {
        self.holder_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerifyAccountDetailsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`routing_number`](VerifyAccountDetailsRequestBuilder::routing_number)
    /// - [`account_number`](VerifyAccountDetailsRequestBuilder::account_number)
    pub fn build(self) -> Result<VerifyAccountDetailsRequest, BuildError> {
        Ok(VerifyAccountDetailsRequest {
            routing_number: self
                .routing_number
                .ok_or_else(|| BuildError::missing_field("routing_number"))?,
            account_number: self
                .account_number
                .ok_or_else(|| BuildError::missing_field("account_number"))?,
            account_type: self.account_type,
            country: self.country,
            account_holder_type: self.account_holder_type,
            holder_name: self.holder_name,
        })
    }
}
