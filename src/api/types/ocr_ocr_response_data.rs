pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrResponseData {
    #[serde(rename = "resultData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_data: Option<OcrResultData>,
}
