pub use crate::prelude::*;

/// A file attached to a case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachmentResponse {
    /// The attachment's identifier.
    #[serde(default)]
    pub uuid: String,
    /// The case the attachment belongs to.
    #[serde(rename = "caseUuid")]
    #[serde(default)]
    pub case_uuid: String,
    /// The file's content type.
    #[serde(rename = "fileType")]
    #[serde(default)]
    pub file_type: String,
    /// The file's name.
    #[serde(default)]
    pub filename: String,
    /// A reference to the stored file.
    #[serde(rename = "fileUrl")]
    #[serde(default)]
    pub file_url: String,
    /// When the file was uploaded.
    #[serde(rename = "uploadedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub uploaded_at: DateTime<Utc>,
    /// The id of the user who uploaded the file.
    #[serde(rename = "uploadedBy")]
    #[serde(default)]
    pub uploaded_by: String,
    /// The resolved user who uploaded the file. Null when not enriched.
    #[serde(rename = "uploadedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_by_user: Option<UserRef>,
}

impl AttachmentResponse {
    pub fn builder() -> AttachmentResponseBuilder {
        <AttachmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentResponseBuilder {
    uuid: Option<String>,
    case_uuid: Option<String>,
    file_type: Option<String>,
    filename: Option<String>,
    file_url: Option<String>,
    uploaded_at: Option<DateTime<Utc>>,
    uploaded_by: Option<String>,
    uploaded_by_user: Option<UserRef>,
}

impl AttachmentResponseBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn case_uuid(mut self, value: impl Into<String>) -> Self {
        self.case_uuid = Some(value.into());
        self
    }

    pub fn file_type(mut self, value: impl Into<String>) -> Self {
        self.file_type = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn uploaded_at(mut self, value: DateTime<Utc>) -> Self {
        self.uploaded_at = Some(value);
        self
    }

    pub fn uploaded_by(mut self, value: impl Into<String>) -> Self {
        self.uploaded_by = Some(value.into());
        self
    }

    pub fn uploaded_by_user(mut self, value: UserRef) -> Self {
        self.uploaded_by_user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](AttachmentResponseBuilder::uuid)
    /// - [`case_uuid`](AttachmentResponseBuilder::case_uuid)
    /// - [`file_type`](AttachmentResponseBuilder::file_type)
    /// - [`filename`](AttachmentResponseBuilder::filename)
    /// - [`file_url`](AttachmentResponseBuilder::file_url)
    /// - [`uploaded_at`](AttachmentResponseBuilder::uploaded_at)
    /// - [`uploaded_by`](AttachmentResponseBuilder::uploaded_by)
    pub fn build(self) -> Result<AttachmentResponse, BuildError> {
        Ok(AttachmentResponse {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            case_uuid: self
                .case_uuid
                .ok_or_else(|| BuildError::missing_field("case_uuid"))?,
            file_type: self
                .file_type
                .ok_or_else(|| BuildError::missing_field("file_type"))?,
            filename: self
                .filename
                .ok_or_else(|| BuildError::missing_field("filename"))?,
            file_url: self
                .file_url
                .ok_or_else(|| BuildError::missing_field("file_url"))?,
            uploaded_at: self
                .uploaded_at
                .ok_or_else(|| BuildError::missing_field("uploaded_at"))?,
            uploaded_by: self
                .uploaded_by
                .ok_or_else(|| BuildError::missing_field("uploaded_by"))?,
            uploaded_by_user: self.uploaded_by_user,
        })
    }
}
