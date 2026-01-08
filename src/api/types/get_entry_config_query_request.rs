pub use crate::prelude::*;

/// Query parameters for getEntryConfig
///
/// Request type for the GetEntryConfigQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEntryConfigQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypages: Option<String>,
}
