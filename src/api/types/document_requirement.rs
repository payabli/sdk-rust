pub use crate::prelude::*;

/// A document the application asks for. A requirement comes from the application's template or from a Payabli business rule. For example, a business with an annual revenue of $1,000,000 or more needs bank statements.
///
/// A person requirement appears once for each owner of the business, with that owner's `personReference`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DocumentRequirement {
    /// The requirement's identifier, such as `documents.bankStatement` or `person.documents.w9`. Send it as `fieldPath` when you upload a document for this requirement.
    #[serde(rename = "fieldPath")]
    #[serde(default)]
    pub field_path: String,
    /// The requirement's display name.
    #[serde(default)]
    pub label: String,
    /// Guidance on what to upload. Omitted when the requirement has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    /// Where the requirement comes from:
    ///
    /// - `template`: the application's template.
    /// - `businessRule`: a Payabli rule, such as bank statements for a business with an annual revenue of $1,000,000 or more.
    pub source: DocumentRequirementSource,
    /// The subjects a document for this requirement can be about. Send one of them as `subject` when you upload.
    #[serde(rename = "subjectTypes")]
    #[serde(default)]
    pub subject_types: Vec<DocumentSubjectType>,
    /// The person this requirement is for. Returned for person requirements only.
    #[serde(rename = "personReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_reference: Option<String>,
    /// When `true`, validation reports the requirement as missing until at least one document is uploaded for it. Submit doesn't enforce document requirements.
    #[serde(default)]
    pub required: bool,
    /// The most documents you can upload for this requirement.
    #[serde(rename = "maxUploads")]
    #[serde(default)]
    pub max_uploads: i64,
    /// The file formats this requirement accepts, such as `pdf`, `jpg`, `jpeg`, or `png`.
    #[serde(rename = "allowedFormats")]
    #[serde(default)]
    pub allowed_formats: Vec<String>,
    /// The number of documents uploaded for this requirement.
    #[serde(rename = "uploadedCount")]
    #[serde(default)]
    pub uploaded_count: i64,
}

impl DocumentRequirement {
    pub fn builder() -> DocumentRequirementBuilder {
        <DocumentRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentRequirementBuilder {
    field_path: Option<String>,
    label: Option<String>,
    hint: Option<String>,
    source: Option<DocumentRequirementSource>,
    subject_types: Option<Vec<DocumentSubjectType>>,
    person_reference: Option<String>,
    required: Option<bool>,
    max_uploads: Option<i64>,
    allowed_formats: Option<Vec<String>>,
    uploaded_count: Option<i64>,
}

impl DocumentRequirementBuilder {
    pub fn field_path(mut self, value: impl Into<String>) -> Self {
        self.field_path = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn hint(mut self, value: impl Into<String>) -> Self {
        self.hint = Some(value.into());
        self
    }

    pub fn source(mut self, value: DocumentRequirementSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn subject_types(mut self, value: Vec<DocumentSubjectType>) -> Self {
        self.subject_types = Some(value);
        self
    }

    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn max_uploads(mut self, value: i64) -> Self {
        self.max_uploads = Some(value);
        self
    }

    pub fn allowed_formats(mut self, value: Vec<String>) -> Self {
        self.allowed_formats = Some(value);
        self
    }

    pub fn uploaded_count(mut self, value: i64) -> Self {
        self.uploaded_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentRequirement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_path`](DocumentRequirementBuilder::field_path)
    /// - [`label`](DocumentRequirementBuilder::label)
    /// - [`source`](DocumentRequirementBuilder::source)
    /// - [`subject_types`](DocumentRequirementBuilder::subject_types)
    /// - [`required`](DocumentRequirementBuilder::required)
    /// - [`max_uploads`](DocumentRequirementBuilder::max_uploads)
    /// - [`allowed_formats`](DocumentRequirementBuilder::allowed_formats)
    /// - [`uploaded_count`](DocumentRequirementBuilder::uploaded_count)
    pub fn build(self) -> Result<DocumentRequirement, BuildError> {
        Ok(DocumentRequirement {
            field_path: self
                .field_path
                .ok_or_else(|| BuildError::missing_field("field_path"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            hint: self.hint,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            subject_types: self
                .subject_types
                .ok_or_else(|| BuildError::missing_field("subject_types"))?,
            person_reference: self.person_reference,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            max_uploads: self
                .max_uploads
                .ok_or_else(|| BuildError::missing_field("max_uploads"))?,
            allowed_formats: self
                .allowed_formats
                .ok_or_else(|| BuildError::missing_field("allowed_formats"))?,
            uploaded_count: self
                .uploaded_count
                .ok_or_else(|| BuildError::missing_field("uploaded_count"))?,
        })
    }
}
