pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an updated business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessUpdatedEnvelope {
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
    /// The updated business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessUpdatedResponse>,
    /// Non-blocking warnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessUpdatedEnvelope {
    pub fn builder() -> BusinessUpdatedEnvelopeBuilder {
        <BusinessUpdatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessUpdatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<BusinessUpdatedResponse>,
    warnings: Option<Vec<BoardingWarning>>,
}

impl BusinessUpdatedEnvelopeBuilder {
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

    pub fn data(mut self, value: BusinessUpdatedResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<BoardingWarning>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessUpdatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](BusinessUpdatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<BusinessUpdatedEnvelope, BuildError> {
        Ok(BusinessUpdatedEnvelope {
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
