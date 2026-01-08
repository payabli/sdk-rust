pub use crate::prelude::*;

/// Request model for check capture processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckCaptureRequest {
    #[serde(rename = "entryPoint")]
    pub entry_point: Entry,
    /// Base64-encoded image of the front of the check. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "frontImage")]
    pub front_image: String,
    /// Base64-encoded image of the back of the check. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "rearImage")]
    pub rear_image: String,
    /// Check amount in cents (maximum 32-bit integer value). For example, $125.50 is represented as 12550.
    #[serde(rename = "checkAmount")]
    pub check_amount: i64,
}
