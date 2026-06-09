pub use crate::prelude::*;

/// Bank account information for an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutBankAccount {
    /// The masked bank account number.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The bank routing number.
    #[serde(rename = "routingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /// The bank name.
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
}

impl TransferOutBankAccount {
    pub fn builder() -> TransferOutBankAccountBuilder {
        <TransferOutBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutBankAccountBuilder {
    account_number: Option<String>,
    routing_number: Option<String>,
    bank_name: Option<String>,
}

impl TransferOutBankAccountBuilder {
    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn routing_number(mut self, value: impl Into<String>) -> Self {
        self.routing_number = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutBankAccount`].
    pub fn build(self) -> Result<TransferOutBankAccount, BuildError> {
        Ok(TransferOutBankAccount {
            account_number: self.account_number,
            routing_number: self.routing_number,
            bank_name: self.bank_name,
        })
    }
}
