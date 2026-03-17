pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliApiResponseOcr {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
    #[serde(rename = "responseCode")]
    pub response_code: Responsecode,
    /// Details of the OCR processing result
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<OcrResponseData>,
}
