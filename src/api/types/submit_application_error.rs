pub use crate::prelude::*;

/// The error body when an application can't be submitted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitApplicationError {
    /// Whether the request succeeded. Always `false`.
    #[serde(default)]
    pub success: bool,
    /// The error code, such as `VALIDATION_ERROR`, `SIGNER_REQUIRED`, or `SIGNER_NOT_OWNER`.
    #[serde(default)]
    pub error: String,
    /// A human-readable description of the error.
    #[serde(default)]
    pub message: String,
    /// The trace identifier for the request, useful for support.
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl SubmitApplicationError {
    pub fn builder() -> SubmitApplicationErrorBuilder {
        <SubmitApplicationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitApplicationErrorBuilder {
    success: Option<bool>,
    error: Option<String>,
    message: Option<String>,
    trace_id: Option<String>,
}

impl SubmitApplicationErrorBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
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

    /// Consumes the builder and constructs a [`SubmitApplicationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SubmitApplicationErrorBuilder::success)
    /// - [`error`](SubmitApplicationErrorBuilder::error)
    /// - [`message`](SubmitApplicationErrorBuilder::message)
    pub fn build(self) -> Result<SubmitApplicationError, BuildError> {
        Ok(SubmitApplicationError {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            trace_id: self.trace_id,
        })
    }
}
