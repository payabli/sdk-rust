pub use crate::prelude::*;

/// Object with detailed error context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliErrorBodyResponseData {
    /// Human-readable explanation of what happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Suggested resolution.
    #[serde(rename = "todoAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_action: Option<String>,
}

impl PayabliErrorBodyResponseData {
    pub fn builder() -> PayabliErrorBodyResponseDataBuilder {
        <PayabliErrorBodyResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliErrorBodyResponseDataBuilder {
    explanation: Option<String>,
    todo_action: Option<String>,
}

impl PayabliErrorBodyResponseDataBuilder {
    pub fn explanation(mut self, value: impl Into<String>) -> Self {
        self.explanation = Some(value.into());
        self
    }

    pub fn todo_action(mut self, value: impl Into<String>) -> Self {
        self.todo_action = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayabliErrorBodyResponseData`].
    pub fn build(self) -> Result<PayabliErrorBodyResponseData, BuildError> {
        Ok(PayabliErrorBodyResponseData {
            explanation: self.explanation,
            todo_action: self.todo_action,
        })
    }
}
