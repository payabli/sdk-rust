pub use crate::prelude::*;

/// Response data from payment processor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V2TransactionDetailResponseData {
    #[serde(rename = "resultCode")]
    pub result_code: ResultCodev2,
    #[serde(rename = "resultCodeText")]
    pub result_code_text: ResultCodeText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    pub responsetext: Resulttext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authcode: Option<Authcode>,
    /// Unique identifier for the transaction assigned by the payment processor.
    pub transactionid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse: Option<AvsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse_text: Option<AvsResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse: Option<CvvResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse_text: Option<CvvResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Processor-specific response code.
    pub response_code: String,
    /// Description of the response code.
    pub response_code_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vault_id: Option<CustomerVaultId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_response_data: Option<EmvAuthResponseData>,
}
