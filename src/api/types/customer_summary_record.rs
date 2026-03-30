pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl CustomerSummaryRecord {
    pub fn builder() -> CustomerSummaryRecordBuilder {
        <CustomerSummaryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerSummaryRecordBuilder {
    numberof_transactions: Option<i64>,
    recent_transactions: Option<Vec<TransactionQueryRecords>>,
    total_amount_transactions: Option<f64>,
    total_net_amount_transactions: Option<f64>,
}

impl CustomerSummaryRecordBuilder {
    pub fn numberof_transactions(mut self, value: i64) -> Self {
        self.numberof_transactions = Some(value);
        self
    }

    pub fn recent_transactions(mut self, value: Vec<TransactionQueryRecords>) -> Self {
        self.recent_transactions = Some(value);
        self
    }

    pub fn total_amount_transactions(mut self, value: f64) -> Self {
        self.total_amount_transactions = Some(value);
        self
    }

    pub fn total_net_amount_transactions(mut self, value: f64) -> Self {
        self.total_net_amount_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerSummaryRecord`].
    pub fn build(self) -> Result<CustomerSummaryRecord, BuildError> {
        Ok(CustomerSummaryRecord {
            numberof_transactions: self.numberof_transactions,
            recent_transactions: self.recent_transactions,
            total_amount_transactions: self.total_amount_transactions,
            total_net_amount_transactions: self.total_net_amount_transactions,
        })
    }
}
