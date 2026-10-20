pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a created business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessCreatedEnvelope {
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
    /// The created business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessCreatedResponse>,
    /// Non-blocking warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessCreatedEnvelope {
    pub fn builder() -> BusinessCreatedEnvelopeBuilder {
        <BusinessCreatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessCreatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<BusinessCreatedResponse>,
    warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessCreatedEnvelopeBuilder {
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

    pub fn data(mut self, value: BusinessCreatedResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<BoardingWarning>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessCreatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](BusinessCreatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<BusinessCreatedEnvelope, BuildError> {
        Ok(BusinessCreatedEnvelope {
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
