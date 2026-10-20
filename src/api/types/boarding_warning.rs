pub use crate::prelude::*;

/// A non-blocking warning returned alongside a successful response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BoardingWarning {
    /// A machine-readable warning code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable description of the warning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BoardingWarning {
    pub fn builder() -> BoardingWarningBuilder {
        <BoardingWarningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingWarningBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl BoardingWarningBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BoardingWarning`].
    pub fn build(self) -> Result<BoardingWarning, BuildError> {
        Ok(BoardingWarning {
            code: self.code,
            message: self.message,
        })
    }
}
