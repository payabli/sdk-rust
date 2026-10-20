pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadApplicationDocumentRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(rename = "fieldPath")]
    #[serde(default)]
    pub field_path: String,
    pub subject: DocumentSubjectType,
    #[serde(rename = "personReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_reference: Option<String>,
}
impl UploadApplicationDocumentRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        form = form.text("fieldPath", self.field_path.clone());

        if let Ok(json_str) = serde_json::to_string(&self.subject) {
            form = form.text("subject", json_str);
        }

        if let Some(ref value) = self.person_reference {
            form = form.text("personReference", value.clone());
        }

        form
    }
}

impl UploadApplicationDocumentRequest {
    pub fn builder() -> UploadApplicationDocumentRequestBuilder {
        <UploadApplicationDocumentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadApplicationDocumentRequestBuilder {
    file: Option<Vec<u8>>,
    field_path: Option<String>,
    subject: Option<DocumentSubjectType>,
    person_reference: Option<String>,
}

impl UploadApplicationDocumentRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn field_path(mut self, value: impl Into<String>) -> Self {
        self.field_path = Some(value.into());
        self
    }

    pub fn subject(mut self, value: DocumentSubjectType) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadApplicationDocumentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadApplicationDocumentRequestBuilder::file)
    /// - [`field_path`](UploadApplicationDocumentRequestBuilder::field_path)
    /// - [`subject`](UploadApplicationDocumentRequestBuilder::subject)
    pub fn build(self) -> Result<UploadApplicationDocumentRequest, BuildError> {
        Ok(UploadApplicationDocumentRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            field_path: self
                .field_path
                .ok_or_else(|| BuildError::missing_field("field_path"))?,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            person_reference: self.person_reference,
        })
    }
}
