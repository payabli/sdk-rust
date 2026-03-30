pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImportVendorRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl ImportVendorRequest {
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

impl ImportVendorRequest {
    pub fn builder() -> ImportVendorRequestBuilder {
        <ImportVendorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportVendorRequestBuilder {
    file: Option<Vec<u8>>,
}

impl ImportVendorRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportVendorRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](ImportVendorRequestBuilder::file)
    pub fn build(self) -> Result<ImportVendorRequest, BuildError> {
        Ok(ImportVendorRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
