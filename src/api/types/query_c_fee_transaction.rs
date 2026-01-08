pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
