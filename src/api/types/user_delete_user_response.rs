pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteUserResponse {
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
