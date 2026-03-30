pub use crate::prelude::*;

/// Individual field error detail for bad request responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V2BadRequestErrorDetail {
    /// Description of the validation error.
    #[serde(default)]
    pub message: String,
    /// Suggested action to fix the error.
    #[serde(default)]
    pub suggestion: String,
}

impl V2BadRequestErrorDetail {
    pub fn builder() -> V2BadRequestErrorDetailBuilder {
        <V2BadRequestErrorDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V2BadRequestErrorDetailBuilder {
    message: Option<String>,
    suggestion: Option<String>,
}

impl V2BadRequestErrorDetailBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn suggestion(mut self, value: impl Into<String>) -> Self {
        self.suggestion = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V2BadRequestErrorDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](V2BadRequestErrorDetailBuilder::message)
    /// - [`suggestion`](V2BadRequestErrorDetailBuilder::suggestion)
    pub fn build(self) -> Result<V2BadRequestErrorDetail, BuildError> {
        Ok(V2BadRequestErrorDetail {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            suggestion: self
                .suggestion
                .ok_or_else(|| BuildError::missing_field("suggestion"))?,
        })
    }
}
