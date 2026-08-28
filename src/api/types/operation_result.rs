pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OperationResult {
    /// Indicates whether the operation was successful.
    #[serde(default)]
    pub success: bool,
    /// A status message describing the result.
    #[serde(default)]
    pub message: String,
    /// The secure link the vendor uses to view their virtual card details. Empty when the operation fails.
    #[serde(default)]
    pub link: String,
}

impl OperationResult {
    pub fn builder() -> OperationResultBuilder {
        <OperationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OperationResultBuilder {
    success: Option<bool>,
    message: Option<String>,
    link: Option<String>,
}

impl OperationResultBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OperationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](OperationResultBuilder::success)
    /// - [`message`](OperationResultBuilder::message)
    /// - [`link`](OperationResultBuilder::link)
    pub fn build(self) -> Result<OperationResult, BuildError> {
        Ok(OperationResult {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
        })
    }
}
