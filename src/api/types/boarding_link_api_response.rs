pub use crate::prelude::*;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BoardingLinkApiResponse {
    /// Reference name for boarding link (if responseText = Success) or
    /// List of empty fields separated by comma (if responseText = Fail)
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
