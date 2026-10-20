pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationEnvelope {
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
    /// The application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Application>,
}

impl ApplicationEnvelope {
    pub fn builder() -> ApplicationEnvelopeBuilder {
        <ApplicationEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<Application>,
}

impl ApplicationEnvelopeBuilder {
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

    pub fn data(mut self, value: Application) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ApplicationEnvelopeBuilder::success)
    pub fn build(self) -> Result<ApplicationEnvelope, BuildError> {
        Ok(ApplicationEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
