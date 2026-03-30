pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OperationResult {
    /// Message describing the result. If the virtual card link was sent successfully, this contains the email address to which the link was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Indicates whether the operation was successful.
    #[serde(default)]
    pub success: bool,
}

impl OperationResult {
    pub fn builder() -> OperationResultBuilder {
        <OperationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OperationResultBuilder {
    message: Option<String>,
    success: Option<bool>,
}

impl OperationResultBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OperationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](OperationResultBuilder::success)
    pub fn build(self) -> Result<OperationResult, BuildError> {
        Ok(OperationResult {
            message: self.message,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
