pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImportCustomerRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(rename = "replaceExisting")]
    #[serde(skip)]
    pub replace_existing: Option<i64>,
}
impl ImportCustomerRequest {
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

impl ImportCustomerRequest {
    pub fn builder() -> ImportCustomerRequestBuilder {
        <ImportCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportCustomerRequestBuilder {
    file: Option<Vec<u8>>,
    replace_existing: Option<i64>,
}

impl ImportCustomerRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn replace_existing(mut self, value: i64) -> Self {
        self.replace_existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportCustomerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](ImportCustomerRequestBuilder::file)
    pub fn build(self) -> Result<ImportCustomerRequest, BuildError> {
        Ok(ImportCustomerRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            replace_existing: self.replace_existing,
        })
    }
}
