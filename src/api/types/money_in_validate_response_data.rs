pub use crate::prelude::*;

/// Response data for card validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidateResponseData {
    #[serde(rename = "authCode")]
    pub auth_code: Authcode,
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
