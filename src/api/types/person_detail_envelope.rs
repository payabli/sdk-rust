pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a person detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PersonDetailEnvelope {
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
    /// The person detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PersonDetailResponse>,
    /// Non-blocking warnings, such as a business link that didn't complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<BoardingWarning>>,
}

impl PersonDetailEnvelope {
    pub fn builder() -> PersonDetailEnvelopeBuilder {
        <PersonDetailEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonDetailEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<PersonDetailResponse>,
    warnings: Option<Vec<BoardingWarning>>,
}

impl PersonDetailEnvelopeBuilder {
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

    pub fn data(mut self, value: PersonDetailResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<BoardingWarning>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonDetailEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](PersonDetailEnvelopeBuilder::success)
    pub fn build(self) -> Result<PersonDetailEnvelope, BuildError> {
        Ok(PersonDetailEnvelope {
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
