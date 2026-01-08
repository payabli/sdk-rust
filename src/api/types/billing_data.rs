pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingData {
    /// Account number for bank account.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Describes whether the bank account is used for deposits or withdrawals in Payabli:
    /// - `0`: Deposit
    /// - `1`: Withdrawal
    /// - `2`: Deposit and withdrawal
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<BankAccountHolderName>,
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<BankAccountHolderType>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<BankName>,
    /// The bank's ID in Payabli.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<RoutingAccount>,
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<TypeAccount>,
}
