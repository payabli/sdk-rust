pub use crate::prelude::*;

/// A document uploaded to an application.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDocument {
    /// The document reference. Use it to delete the document.
    #[serde(rename = "documentReference")]
    #[serde(default)]
    pub document_reference: String,
    /// How the document was added to the application. Documents you upload are `applicantUpload`.
    #[serde(default)]
    pub kind: String,
    /// The record the document is about.
    #[serde(rename = "subjectType")]
    pub subject_type: DocumentSubjectType,
    /// The person the document is about, when `subjectType` is `person`. Otherwise, `null`.
    #[serde(rename = "personReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_reference: Option<String>,
    /// The requirement the document was uploaded for.
    #[serde(rename = "fieldPath")]
    #[serde(default)]
    pub field_path: String,
    /// The name of the uploaded file.
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    /// The file's media type, such as `application/pdf`.
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
    /// The file's size, in bytes.
    #[serde(rename = "fileSizeBytes")]
    #[serde(default)]
    pub file_size_bytes: i64,
    /// When the document was uploaded.
    #[serde(rename = "uploadedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub uploaded_at: DateTime<Utc>,
}

impl ApplicationDocument {
    pub fn builder() -> ApplicationDocumentBuilder {
        <ApplicationDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDocumentBuilder {
    document_reference: Option<String>,
    kind: Option<String>,
    subject_type: Option<DocumentSubjectType>,
    person_reference: Option<String>,
    field_path: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size_bytes: Option<i64>,
    uploaded_at: Option<DateTime<Utc>>,
}

impl ApplicationDocumentBuilder {
    pub fn document_reference(mut self, value: impl Into<String>) -> Self {
        self.document_reference = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn subject_type(mut self, value: DocumentSubjectType) -> Self {
        self.subject_type = Some(value);
        self
    }

    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn field_path(mut self, value: impl Into<String>) -> Self {
        self.field_path = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn file_size_bytes(mut self, value: i64) -> Self {
        self.file_size_bytes = Some(value);
        self
    }

    pub fn uploaded_at(mut self, value: DateTime<Utc>) -> Self {
        self.uploaded_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_reference`](ApplicationDocumentBuilder::document_reference)
    /// - [`kind`](ApplicationDocumentBuilder::kind)
    /// - [`subject_type`](ApplicationDocumentBuilder::subject_type)
    /// - [`field_path`](ApplicationDocumentBuilder::field_path)
    /// - [`file_name`](ApplicationDocumentBuilder::file_name)
    /// - [`mime_type`](ApplicationDocumentBuilder::mime_type)
    /// - [`file_size_bytes`](ApplicationDocumentBuilder::file_size_bytes)
    /// - [`uploaded_at`](ApplicationDocumentBuilder::uploaded_at)
    pub fn build(self) -> Result<ApplicationDocument, BuildError> {
        Ok(ApplicationDocument {
            document_reference: self
                .document_reference
                .ok_or_else(|| BuildError::missing_field("document_reference"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            subject_type: self
                .subject_type
                .ok_or_else(|| BuildError::missing_field("subject_type"))?,
            person_reference: self.person_reference,
            field_path: self
                .field_path
                .ok_or_else(|| BuildError::missing_field("field_path"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self
                .mime_type
                .ok_or_else(|| BuildError::missing_field("mime_type"))?,
            file_size_bytes: self
                .file_size_bytes
                .ok_or_else(|| BuildError::missing_field("file_size_bytes"))?,
            uploaded_at: self
                .uploaded_at
                .ok_or_else(|| BuildError::missing_field("uploaded_at"))?,
        })
    }
}
