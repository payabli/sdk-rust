pub use crate::prelude::*;

/// Object that contains bank account details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Bank {
    /// The Payabli-assigned internal identifier for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// A user-defined internal identifier for the bank account. This allows you to specify which bank account should be used for payments in cases where multiple accounts are configured.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<BankNickname>,
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
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<BankAccountHolderType>,
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<BankAccountFunction>,
    /// Bank account verification status. When `true`, the account has been verified to exist and be in good standing based on vendor checks or previous processing histories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Bank account status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Array of services associated with this bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}
