pub use crate::prelude::*;

/// Describes the reason for a failed operation and how to resolve it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseError400ResponseData {
    /// Describes the reason the operation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Describes how to resolve the error.
    #[serde(rename = "todoAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_action: Option<String>,
}

impl PayabliApiResponseError400ResponseData {
    pub fn builder() -> PayabliApiResponseError400ResponseDataBuilder {
        <PayabliApiResponseError400ResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseError400ResponseDataBuilder {
    explanation: Option<String>,
    todo_action: Option<String>,
}

impl PayabliApiResponseError400ResponseDataBuilder {
    pub fn explanation(mut self, value: impl Into<String>) -> Self {
        self.explanation = Some(value.into());
        self
    }

    pub fn todo_action(mut self, value: impl Into<String>) -> Self {
        self.todo_action = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseError400ResponseData`].
    pub fn build(self) -> Result<PayabliApiResponseError400ResponseData, BuildError> {
        Ok(PayabliApiResponseError400ResponseData {
            explanation: self.explanation,
            todo_action: self.todo_action,
        })
    }
}
