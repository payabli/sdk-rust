pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a page of applications.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApplicationsEnvelope {
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
    /// The page of applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListApplicationsResponse>,
}

impl ListApplicationsEnvelope {
    pub fn builder() -> ListApplicationsEnvelopeBuilder {
        <ListApplicationsEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApplicationsEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<ListApplicationsResponse>,
}

impl ListApplicationsEnvelopeBuilder {
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

    pub fn data(mut self, value: ListApplicationsResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListApplicationsEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ListApplicationsEnvelopeBuilder::success)
    pub fn build(self) -> Result<ListApplicationsEnvelope, BuildError> {
        Ok(ListApplicationsEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
