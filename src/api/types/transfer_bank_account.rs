pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferBankAccount {
    #[serde(rename = "accountNumber")]
    #[serde(default)]
    pub account_number: AccountNumber,
    #[serde(rename = "routingNumber")]
    #[serde(default)]
    pub routing_number: RoutingAccount,
    #[serde(rename = "bankName")]
    #[serde(default)]
    pub bank_name: BankName,
}

impl TransferBankAccount {
    pub fn builder() -> TransferBankAccountBuilder {
        <TransferBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBankAccountBuilder {
    account_number: Option<AccountNumber>,
    routing_number: Option<RoutingAccount>,
    bank_name: Option<BankName>,
}

impl TransferBankAccountBuilder {
    pub fn account_number(mut self, value: AccountNumber) -> Self {
        self.account_number = Some(value);
        self
    }

    pub fn routing_number(mut self, value: RoutingAccount) -> Self {
        self.routing_number = Some(value);
        self
    }

    pub fn bank_name(mut self, value: BankName) -> Self {
        self.bank_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferBankAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_number`](TransferBankAccountBuilder::account_number)
    /// - [`routing_number`](TransferBankAccountBuilder::routing_number)
    /// - [`bank_name`](TransferBankAccountBuilder::bank_name)
    pub fn build(self) -> Result<TransferBankAccount, BuildError> {
        Ok(TransferBankAccount {
            account_number: self
                .account_number
                .ok_or_else(|| BuildError::missing_field("account_number"))?,
            routing_number: self
                .routing_number
                .ok_or_else(|| BuildError::missing_field("routing_number"))?,
            bank_name: self
                .bank_name
                .ok_or_else(|| BuildError::missing_field("bank_name"))?,
        })
    }
}
