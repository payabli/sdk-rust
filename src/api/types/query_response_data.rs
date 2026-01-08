pub use crate::prelude::*;

/// The transaction's response data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authcode: Option<Authcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse: Option<AvsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse_text: Option<AvsResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse: Option<CvvResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse_text: Option<CvvResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_response_data: Option<EmvAuthResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<OrderId>,
    /// Response text for operation: 'Success' or 'Declined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// Internal result code processing the transaction. Value 1 indicates successful operation, values 2 and 3 indicate errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// Text describing the result. If resultCode = 1, will return 'Approved' or a general success message. If resultCode = 2 or 3, will contain the cause of the decline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code_text: Option<String>,
    /// Text describing the result. If resultCode = 1, will return 'Approved' or a general success message. If resultCode = 2 or 3, will contain the cause of the decline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsetext: Option<String>,
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCodev2>,
    #[serde(rename = "resultCodeText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code_text: Option<ResultCodeText>,
    /// The transaction identifier in Payabli.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionid: Option<String>,
    /// Type of transaction or operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
