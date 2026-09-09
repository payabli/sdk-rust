pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OcrDocumentFormRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl OcrDocumentFormRequest {
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

impl OcrDocumentFormRequest {
    pub fn builder() -> OcrDocumentFormRequestBuilder {
        <OcrDocumentFormRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrDocumentFormRequestBuilder {
    file: Option<Vec<u8>>,
}

impl OcrDocumentFormRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrDocumentFormRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](OcrDocumentFormRequestBuilder::file)
    pub fn build(self) -> Result<OcrDocumentFormRequest, BuildError> {
        Ok(OcrDocumentFormRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
