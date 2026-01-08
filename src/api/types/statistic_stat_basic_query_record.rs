pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatBasicQueryRecord {
    /// Statistical grouping identifier
    #[serde(rename = "statX")]
    pub stat_x: String,
    /// Number of incoming transactions
    #[serde(rename = "inTransactions")]
    pub in_transactions: i64,
    /// Volume of incoming transactions
    #[serde(rename = "inTransactionsVolume")]
    pub in_transactions_volume: f64,
    /// Number of incoming wallet transactions
    #[serde(rename = "inWalletTransactions")]
    pub in_wallet_transactions: i64,
    /// Volume of incoming wallet transactions
    #[serde(rename = "inWalletVolume")]
    pub in_wallet_volume: f64,
}
