pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerSummaryRecord {
    /// Number total of transactions or payments
    #[serde(rename = "numberofTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numberof_transactions: Option<i64>,
    /// List of more recent 5 transactions belonging to the customer
    #[serde(rename = "recentTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_transactions: Option<Vec<TransactionQueryRecords>>,
    /// Total amount in transactions
    #[serde(rename = "totalAmountTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount_transactions: Option<f64>,
    /// Total net amount in transactions
    #[serde(rename = "totalNetAmountTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount_transactions: Option<f64>,
}
