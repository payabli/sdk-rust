pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillingDataResponse {
    /// The bank's ID in Payabli.
    pub id: i64,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<serde_json::Value>,
    pub nickname: String,
    #[serde(rename = "bankName")]
    pub bank_name: BankName,
    #[serde(rename = "routingAccount")]
    pub routing_account: RoutingAccount,
    #[serde(rename = "accountNumber")]
    pub account_number: AccountNumber,
    #[serde(rename = "typeAccount")]
    pub type_account: TypeAccount,
    #[serde(rename = "bankAccountHolderName")]
    pub bank_account_holder_name: BankAccountHolderName,
    #[serde(rename = "bankAccountHolderType")]
    pub bank_account_holder_type: BankAccountHolderType,
    /// Describes whether the bank account is used for deposits or withdrawals in Payabli:
    /// - `0`: Deposit
    /// - `1`: Withdrawal
    /// - `2`: Deposit and withdrawal
    #[serde(rename = "bankAccountFunction")]
    pub bank_account_function: i64,
    pub verified: bool,
    pub status: i64,
    pub services: Vec<serde_json::Value>,
    pub default: bool,
}
