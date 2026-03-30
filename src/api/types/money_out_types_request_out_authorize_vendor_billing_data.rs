pub use crate::prelude::*;

/// Object containing vendor's bank information. This object is deprecated for this endpoint. Use the `paymentMethod` object in payout authorize requests instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestOutAuthorizeVendorBillingData {
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
}

impl RequestOutAuthorizeVendorBillingData {
    pub fn builder() -> RequestOutAuthorizeVendorBillingDataBuilder {
        <RequestOutAuthorizeVendorBillingDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeVendorBillingDataBuilder {
    bank_name: Option<BankName>,
    routing_account: Option<RoutingAccount>,
    account_number: Option<AccountNumber>,
    type_account: Option<TypeAccount>,
    bank_account_holder_name: Option<BankAccountHolderName>,
}

impl RequestOutAuthorizeVendorBillingDataBuilder {
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

    /// Consumes the builder and constructs a [`RequestOutAuthorizeVendorBillingData`].
    pub fn build(self) -> Result<RequestOutAuthorizeVendorBillingData, BuildError> {
        Ok(RequestOutAuthorizeVendorBillingData {
            bank_name: self.bank_name,
            routing_account: self.routing_account,
            account_number: self.account_number,
            type_account: self.type_account,
            bank_account_holder_name: self.bank_account_holder_name,
        })
    }
}
