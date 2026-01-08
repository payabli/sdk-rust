pub use crate::prelude::*;

/// Query parameters for getExternalApplication
///
/// Request type for the GetExternalApplicationQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetExternalApplicationQueryRequest {
    /// If `true`, sends an email that includes the link to the application to the `mail2` address. Defaults to `false`.
    #[serde(rename = "sendEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email: Option<bool>,
}
