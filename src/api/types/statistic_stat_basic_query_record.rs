pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatBasicQueryRecord {
    /// Statistical grouping identifier
    #[serde(rename = "statX")]
    #[serde(default)]
    pub stat_x: String,
    /// Number of incoming transactions
    #[serde(rename = "inTransactions")]
    #[serde(default)]
    pub in_transactions: i64,
    /// Volume of incoming transactions
    #[serde(rename = "inTransactionsVolume")]
    #[serde(default)]
    pub in_transactions_volume: f64,
    /// Number of incoming wallet transactions
    #[serde(rename = "inWalletTransactions")]
    #[serde(default)]
    pub in_wallet_transactions: i64,
    /// Volume of incoming wallet transactions
    #[serde(rename = "inWalletVolume")]
    #[serde(default)]
    pub in_wallet_volume: f64,
}

impl StatBasicQueryRecord {
    pub fn builder() -> StatBasicQueryRecordBuilder {
        <StatBasicQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatBasicQueryRecordBuilder {
    stat_x: Option<String>,
    in_transactions: Option<i64>,
    in_transactions_volume: Option<f64>,
    in_wallet_transactions: Option<i64>,
    in_wallet_volume: Option<f64>,
}

impl StatBasicQueryRecordBuilder {
    pub fn stat_x(mut self, value: impl Into<String>) -> Self {
        self.stat_x = Some(value.into());
        self
    }

    pub fn in_transactions(mut self, value: i64) -> Self {
        self.in_transactions = Some(value);
        self
    }

    pub fn in_transactions_volume(mut self, value: f64) -> Self {
        self.in_transactions_volume = Some(value);
        self
    }

    pub fn in_wallet_transactions(mut self, value: i64) -> Self {
        self.in_wallet_transactions = Some(value);
        self
    }

    pub fn in_wallet_volume(mut self, value: f64) -> Self {
        self.in_wallet_volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatBasicQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stat_x`](StatBasicQueryRecordBuilder::stat_x)
    /// - [`in_transactions`](StatBasicQueryRecordBuilder::in_transactions)
    /// - [`in_transactions_volume`](StatBasicQueryRecordBuilder::in_transactions_volume)
    /// - [`in_wallet_transactions`](StatBasicQueryRecordBuilder::in_wallet_transactions)
    /// - [`in_wallet_volume`](StatBasicQueryRecordBuilder::in_wallet_volume)
    pub fn build(self) -> Result<StatBasicQueryRecord, BuildError> {
        Ok(StatBasicQueryRecord {
            stat_x: self
                .stat_x
                .ok_or_else(|| BuildError::missing_field("stat_x"))?,
            in_transactions: self
                .in_transactions
                .ok_or_else(|| BuildError::missing_field("in_transactions"))?,
            in_transactions_volume: self
                .in_transactions_volume
                .ok_or_else(|| BuildError::missing_field("in_transactions_volume"))?,
            in_wallet_transactions: self
                .in_wallet_transactions
                .ok_or_else(|| BuildError::missing_field("in_wallet_transactions"))?,
            in_wallet_volume: self
                .in_wallet_volume
                .ok_or_else(|| BuildError::missing_field("in_wallet_volume"))?,
        })
    }
}
