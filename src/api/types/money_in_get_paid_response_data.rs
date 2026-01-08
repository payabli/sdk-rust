pub use crate::prelude::*;

/// Response data for GetPaid transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPaidResponseData {
    #[serde(rename = "authCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
    /// Details of the transaction. Present only if `includeDetails` query parameter is set to `true` in the request.
    #[serde(rename = "transactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetailRecord>,
    #[serde(rename = "referenceId")]
    pub reference_id: Referenceidtrans,
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
    #[serde(rename = "resultText")]
    pub result_text: Resulttext,
    #[serde(rename = "avsResponseText")]
    pub avs_response_text: AvsResponseText,
    #[serde(rename = "cvvResponseText")]
    pub cvv_response_text: CvvResponseText,
    #[serde(rename = "customerId")]
    pub customer_id: Customeridtrans,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}
