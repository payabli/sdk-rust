pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResponseDataRefunds {
    #[serde(rename = "authCode")]
    pub auth_code: Authcode,
    #[serde(rename = "expectedProcessingDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_processing_date_time: Option<ExpectedProcessingDateTime>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "cvvResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_response_text: Option<CvvResponseText>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
    #[serde(rename = "referenceId")]
    pub reference_id: Referenceidtrans,
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
    /// Text description of the transaction result
    #[serde(rename = "resultText")]
    pub result_text: String,
}
