pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImportBillsRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl ImportBillsRequest {
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

impl ImportBillsRequest {
    pub fn builder() -> ImportBillsRequestBuilder {
        <ImportBillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportBillsRequestBuilder {
    file: Option<Vec<u8>>,
}

impl ImportBillsRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportBillsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](ImportBillsRequestBuilder::file)
    pub fn build(self) -> Result<ImportBillsRequest, BuildError> {
        Ok(ImportBillsRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
