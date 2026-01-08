pub use crate::prelude::*;

/// Object containing details about cloud devices and their registration history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CloudQueryApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    /// List of devices and history of registration.
    #[serde(rename = "responseList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_list: Option<Vec<PoiDevice>>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
