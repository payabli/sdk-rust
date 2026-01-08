pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthCapturePayoutResponseData {
    #[serde(rename = "authCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
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
    pub customer_id: Customeridtrans,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}
