pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckCaptureRequestBody {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: Entry,
    /// Base64-encoded front check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check with no partial, blurry, or illegible portions.
    #[serde(rename = "frontImage")]
    #[serde(default)]
    pub front_image: String,
    /// Base64-encoded rear check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check with no partial, blurry, or illegible portions.
    #[serde(rename = "rearImage")]
    #[serde(default)]
    pub rear_image: String,
    /// Check amount in cents (maximum 32-bit integer value).
    #[serde(rename = "checkAmount")]
    #[serde(default)]
    pub check_amount: i64,
}

impl CheckCaptureRequestBody {
    pub fn builder() -> CheckCaptureRequestBodyBuilder {
        <CheckCaptureRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckCaptureRequestBodyBuilder {
    entry_point: Option<Entry>,
    front_image: Option<String>,
    rear_image: Option<String>,
    check_amount: Option<i64>,
}

impl CheckCaptureRequestBodyBuilder {
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

    /// Consumes the builder and constructs a [`CheckCaptureRequestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](CheckCaptureRequestBodyBuilder::entry_point)
    /// - [`front_image`](CheckCaptureRequestBodyBuilder::front_image)
    /// - [`rear_image`](CheckCaptureRequestBodyBuilder::rear_image)
    /// - [`check_amount`](CheckCaptureRequestBodyBuilder::check_amount)
    pub fn build(self) -> Result<CheckCaptureRequestBody, BuildError> {
        Ok(CheckCaptureRequestBody {
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
