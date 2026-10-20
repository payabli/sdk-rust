pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an application's status history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatusHistoryEnvelope {
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
    /// The application's status history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StatusHistoryResult>,
}

impl StatusHistoryEnvelope {
    pub fn builder() -> StatusHistoryEnvelopeBuilder {
        <StatusHistoryEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusHistoryEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<StatusHistoryResult>,
}

impl StatusHistoryEnvelopeBuilder {
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

    pub fn data(mut self, value: StatusHistoryResult) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatusHistoryEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](StatusHistoryEnvelopeBuilder::success)
    pub fn build(self) -> Result<StatusHistoryEnvelope, BuildError> {
        Ok(StatusHistoryEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
