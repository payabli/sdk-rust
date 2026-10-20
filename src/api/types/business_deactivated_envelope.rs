pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a deactivated business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessDeactivatedEnvelope {
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
    /// The deactivated business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessDeactivatedResponse>,
    /// Non-blocking warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessDeactivatedEnvelope {
    pub fn builder() -> BusinessDeactivatedEnvelopeBuilder {
        <BusinessDeactivatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessDeactivatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<BusinessDeactivatedResponse>,
    warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessDeactivatedEnvelopeBuilder {
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

    pub fn data(mut self, value: BusinessDeactivatedResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<BoardingWarning>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessDeactivatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](BusinessDeactivatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<BusinessDeactivatedEnvelope, BuildError> {
        Ok(BusinessDeactivatedEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
            warnings: self.warnings,
        })
    }
}
