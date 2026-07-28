pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadAttachmentRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl UploadAttachmentRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        form
    }
}

impl UploadAttachmentRequest {
    pub fn builder() -> UploadAttachmentRequestBuilder {
        <UploadAttachmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadAttachmentRequestBuilder {
    file: Option<Vec<u8>>,
}

impl UploadAttachmentRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadAttachmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadAttachmentRequestBuilder::file)
    pub fn build(self) -> Result<UploadAttachmentRequest, BuildError> {
        Ok(UploadAttachmentRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
