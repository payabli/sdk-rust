pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatCustomerBasicQueryRecord {
    /// The time bucket for this row, formatted according to the query's `freq` (for example, `2026-9` for a monthly bucket). The response returns one object per bucket across the requested range.
    #[serde(rename = "statX")]
    #[serde(default)]
    pub stat_x: String,
    /// Count of the customer's approved transactions.
    #[serde(rename = "inTransactions")]
    #[serde(default)]
    pub in_transactions: i64,
    /// Total gross value of the customer's approved transactions. Unlike `/Statistic/basic`, this volume is the gross amount, before fees.
    #[serde(rename = "inTransactionsVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_transactions_volume: f64,
}

impl StatCustomerBasicQueryRecord {
    pub fn builder() -> StatCustomerBasicQueryRecordBuilder {
        <StatCustomerBasicQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatCustomerBasicQueryRecordBuilder {
    stat_x: Option<String>,
    in_transactions: Option<i64>,
    in_transactions_volume: Option<f64>,
}

impl StatCustomerBasicQueryRecordBuilder {
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

    /// Consumes the builder and constructs a [`StatCustomerBasicQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stat_x`](StatCustomerBasicQueryRecordBuilder::stat_x)
    /// - [`in_transactions`](StatCustomerBasicQueryRecordBuilder::in_transactions)
    /// - [`in_transactions_volume`](StatCustomerBasicQueryRecordBuilder::in_transactions_volume)
    pub fn build(self) -> Result<StatCustomerBasicQueryRecord, BuildError> {
        Ok(StatCustomerBasicQueryRecord {
            stat_x: self
                .stat_x
                .ok_or_else(|| BuildError::missing_field("stat_x"))?,
            in_transactions: self
                .in_transactions
                .ok_or_else(|| BuildError::missing_field("in_transactions"))?,
            in_transactions_volume: self
                .in_transactions_volume
                .ok_or_else(|| BuildError::missing_field("in_transactions_volume"))?,
        })
    }
}
