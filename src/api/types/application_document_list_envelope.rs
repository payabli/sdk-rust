pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an application's documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDocumentListEnvelope {
    /// Whether the request succeeded.
    #[serde(default)]
    pub success: bool,
    /// A human-readable result message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The trace identifier for the request, useful for support.
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// The documents uploaded to the application. Empty when none are uploaded.
    #[serde(default)]
    pub data: Vec<ApplicationDocument>,
}

impl ApplicationDocumentListEnvelope {
    pub fn builder() -> ApplicationDocumentListEnvelopeBuilder {
        <ApplicationDocumentListEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDocumentListEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<Vec<ApplicationDocument>>,
}

impl ApplicationDocumentListEnvelopeBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn data(mut self, value: Vec<ApplicationDocument>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDocumentListEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ApplicationDocumentListEnvelopeBuilder::success)
    /// - [`data`](ApplicationDocumentListEnvelopeBuilder::data)
    pub fn build(self) -> Result<ApplicationDocumentListEnvelope, BuildError> {
        Ok(ApplicationDocumentListEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
