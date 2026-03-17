pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MigratePaypointResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
