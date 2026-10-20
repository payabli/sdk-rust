pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an uploaded document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApplicationDocumentEnvelope {
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
    pub data: ApplicationDocument,
}

impl ApplicationDocumentEnvelope {
    pub fn builder() -> ApplicationDocumentEnvelopeBuilder {
        <ApplicationDocumentEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDocumentEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<ApplicationDocument>,
}

impl ApplicationDocumentEnvelopeBuilder {
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

    pub fn data(mut self, value: ApplicationDocument) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDocumentEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ApplicationDocumentEnvelopeBuilder::success)
    /// - [`data`](ApplicationDocumentEnvelopeBuilder::data)
    pub fn build(self) -> Result<ApplicationDocumentEnvelope, BuildError> {
        Ok(ApplicationDocumentEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
