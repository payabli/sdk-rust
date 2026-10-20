pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an updated paypoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointUpdatedEnvelope {
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
    /// The updated paypoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PaypointUpdatedResponse>,
}

impl PaypointUpdatedEnvelope {
    pub fn builder() -> PaypointUpdatedEnvelopeBuilder {
        <PaypointUpdatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointUpdatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<PaypointUpdatedResponse>,
}

impl PaypointUpdatedEnvelopeBuilder {
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

    pub fn data(mut self, value: PaypointUpdatedResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointUpdatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](PaypointUpdatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<PaypointUpdatedEnvelope, BuildError> {
        Ok(PaypointUpdatedEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
