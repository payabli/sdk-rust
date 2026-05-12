pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryCFeeTransaction {
    #[serde(rename = "cFeeTransid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_fee_transid: Option<String>,
    #[serde(rename = "feeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "refundId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<i64>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "settlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<i64>,
    #[serde(rename = "transactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<TransactionTime>,
    #[serde(rename = "transStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
}

impl QueryCFeeTransaction {
    pub fn builder() -> QueryCFeeTransactionBuilder {
        <QueryCFeeTransactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryCFeeTransactionBuilder {
    c_fee_transid: Option<String>,
    fee_amount: Option<f64>,
    operation: Option<String>,
    refund_id: Option<i64>,
    response_data: Option<HashMap<String, serde_json::Value>>,
    settlement_status: Option<i64>,
    transaction_time: Option<TransactionTime>,
    trans_status: Option<i64>,
}

impl QueryCFeeTransactionBuilder {
    pub fn c_fee_transid(mut self, value: impl Into<String>) -> Self {
        self.c_fee_transid = Some(value.into());
        self
    }

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    pub fn refund_id(mut self, value: i64) -> Self {
        self.refund_id = Some(value);
        self
    }

    pub fn response_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn settlement_status(mut self, value: i64) -> Self {
        self.settlement_status = Some(value);
        self
    }

    pub fn transaction_time(mut self, value: TransactionTime) -> Self {
        self.transaction_time = Some(value);
        self
    }

    pub fn trans_status(mut self, value: i64) -> Self {
        self.trans_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryCFeeTransaction`].
    pub fn build(self) -> Result<QueryCFeeTransaction, BuildError> {
        Ok(QueryCFeeTransaction {
            c_fee_transid: self.c_fee_transid,
            fee_amount: self.fee_amount,
            operation: self.operation,
            refund_id: self.refund_id,
            response_data: self.response_data,
            settlement_status: self.settlement_status,
            transaction_time: self.transaction_time,
            trans_status: self.trans_status,
        })
    }
}
