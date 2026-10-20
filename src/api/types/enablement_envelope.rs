pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an application's enablement progress.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnablementEnvelope {
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
    /// The application's enablement progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EnablementResult>,
}

impl EnablementEnvelope {
    pub fn builder() -> EnablementEnvelopeBuilder {
        <EnablementEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnablementEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<EnablementResult>,
}

impl EnablementEnvelopeBuilder {
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

    pub fn data(mut self, value: EnablementResult) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnablementEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](EnablementEnvelopeBuilder::success)
    pub fn build(self) -> Result<EnablementEnvelope, BuildError> {
        Ok(EnablementEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
