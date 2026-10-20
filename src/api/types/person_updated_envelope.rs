pub use crate::prelude::*;

/// Standard boarding response envelope wrapping an updated person.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonUpdatedEnvelope {
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
    /// The updated person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PersonUpdatedResponse>,
}

impl PersonUpdatedEnvelope {
    pub fn builder() -> PersonUpdatedEnvelopeBuilder {
        <PersonUpdatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonUpdatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<PersonUpdatedResponse>,
}

impl PersonUpdatedEnvelopeBuilder {
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

    pub fn data(mut self, value: PersonUpdatedResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonUpdatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](PersonUpdatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<PersonUpdatedEnvelope, BuildError> {
        Ok(PersonUpdatedEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
