pub use crate::prelude::*;

/// Billing data for a vendor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutDetailVendorBillingData {
    /// Unique identifier for the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The account ID.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A nickname for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The name of the bank.
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// The routing number.
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<String>,
    /// The account number.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The type of account.
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<String>,
    /// The name of the account holder.
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<String>,
    /// The type of account holder.
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<String>,
    /// The function of the bank account.
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
    /// Whether the account is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The status of the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Services associated with the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<serde_json::Value>>,
    /// Whether this is the default billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The country of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}