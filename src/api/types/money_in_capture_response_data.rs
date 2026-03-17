pub use crate::prelude::*;

/// Response data for capture transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptureResponseData {
    #[serde(rename = "authCode")]
    pub auth_code: Authcode,
    #[serde(rename = "referenceId")]
    pub reference_id: Referenceidtrans,
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
    #[serde(rename = "resultText")]
    pub result_text: Resulttext,
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    #[serde(rename = "cvvResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_response_text: Option<CvvResponseText>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Customeridtrans>,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}
