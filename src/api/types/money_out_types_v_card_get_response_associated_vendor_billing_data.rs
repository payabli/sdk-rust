pub use crate::prelude::*;

/// Billing data for the vendor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VCardGetResponseAssociatedVendorBillingData {
    /// Unique identifier for billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Account identifier.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Nickname for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Name of the bank used for transactions.
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// Routing number for the bank account.
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<String>,
    /// Masked account number for transactions.
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Type of the bank account.
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<String>,
    /// Name of the bank account holder.
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<String>,
    /// Type of bank account holder.
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<String>,
    /// Function of the bank account.
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
    /// Indicates if the account is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Status of the billing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Services associated with the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<serde_json::Value>>,
    /// Indicates if this is the default billing account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}
