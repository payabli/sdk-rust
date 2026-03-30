pub use crate::prelude::*;

/// Request model for check capture processing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckCaptureRequest {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: Entry,
    /// Base64-encoded image of the front of the check. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "frontImage")]
    #[serde(default)]
    pub front_image: String,
    /// Base64-encoded image of the back of the check. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    #[serde(rename = "rearImage")]
    #[serde(default)]
    pub rear_image: String,
    /// Check amount in cents (maximum 32-bit integer value). For example, $125.50 is represented as 12550.
    #[serde(rename = "checkAmount")]
    #[serde(default)]
    pub check_amount: i64,
}

impl CheckCaptureRequest {
    pub fn builder() -> CheckCaptureRequestBuilder {
        <CheckCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckCaptureRequestBuilder {
    entry_point: Option<Entry>,
    front_image: Option<String>,
    rear_image: Option<String>,
    check_amount: Option<i64>,
}

impl CheckCaptureRequestBuilder {
    pub fn entry_point(mut self, value: Entry) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn front_image(mut self, value: impl Into<String>) -> Self {
        self.front_image = Some(value.into());
        self
    }

    pub fn rear_image(mut self, value: impl Into<String>) -> Self {
        self.rear_image = Some(value.into());
        self
    }

    pub fn check_amount(mut self, value: i64) -> Self {
        self.check_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](CheckCaptureRequestBuilder::entry_point)
    /// - [`front_image`](CheckCaptureRequestBuilder::front_image)
    /// - [`rear_image`](CheckCaptureRequestBuilder::rear_image)
    /// - [`check_amount`](CheckCaptureRequestBuilder::check_amount)
    pub fn build(self) -> Result<CheckCaptureRequest, BuildError> {
        Ok(CheckCaptureRequest {
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            front_image: self
                .front_image
                .ok_or_else(|| BuildError::missing_field("front_image"))?,
            rear_image: self
                .rear_image
                .ok_or_else(|| BuildError::missing_field("rear_image"))?,
            check_amount: self
                .check_amount
                .ok_or_else(|| BuildError::missing_field("check_amount"))?,
        })
    }
}
