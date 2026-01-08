pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckCaptureRequestBody {
    #[serde(rename = "entryPoint")]
    pub entry_point: Entry,
    /// Base64-encoded front check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "frontImage")]
    pub front_image: String,
    /// Base64-encoded rear check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "rearImage")]
    pub rear_image: String,
    /// Check amount in cents (maximum 32-bit integer value).
    #[serde(rename = "checkAmount")]
    pub check_amount: i64,
}
