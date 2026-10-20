pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a lifecycle-action result.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransitionResultEnvelope {
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
    /// The lifecycle-action result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TransitionResult>,
}

impl TransitionResultEnvelope {
    pub fn builder() -> TransitionResultEnvelopeBuilder {
        <TransitionResultEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransitionResultEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<TransitionResult>,
}

impl TransitionResultEnvelopeBuilder {
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

    pub fn data(mut self, value: TransitionResult) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransitionResultEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](TransitionResultEnvelopeBuilder::success)
    pub fn build(self) -> Result<TransitionResultEnvelope, BuildError> {
        Ok(TransitionResultEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
