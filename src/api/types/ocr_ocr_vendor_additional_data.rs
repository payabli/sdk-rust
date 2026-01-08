pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrVendorAdditionalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}
