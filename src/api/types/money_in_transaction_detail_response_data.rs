pub use crate::prelude::*;

/// Response data from payment processor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionDetailResponseData {
    /// Unified result code for the transaction. See [Pay In unified response codes](/developers/references/pay-in-unified-response-codes) for more information.
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    /// Description of the result code. See [Pay In unified response codes](/developers/references/pay-in-unified-response-codes) for more information.
    #[serde(rename = "resultCodeText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    pub responsetext: Resulttext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authcode: Option<Authcode>,
    pub transactionid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse_text: Option<AvsResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse_text: Option<CvvResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub response_code: String,
    pub response_code_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vault_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_response_data: Option<EmvAuthResponseData>,
}
