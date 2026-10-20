pub use crate::prelude::*;

/// An application's status history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatusHistoryResult {
    /// The application reference.
    #[serde(rename = "requestsReference")]
    #[serde(default)]
    pub requests_reference: String,
    /// The status transitions, oldest first.
    #[serde(default)]
    pub history: Vec<StatusHistoryEntry>,
}

impl StatusHistoryResult {
    pub fn builder() -> StatusHistoryResultBuilder {
        <StatusHistoryResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusHistoryResultBuilder {
    requests_reference: Option<String>,
    history: Option<Vec<StatusHistoryEntry>>,
}

impl StatusHistoryResultBuilder {
    pub fn requests_reference(mut self, value: impl Into<String>) -> Self {
        self.requests_reference = Some(value.into());
        self
    }

    pub fn history(mut self, value: Vec<StatusHistoryEntry>) -> Self {
        self.history = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatusHistoryResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests_reference`](StatusHistoryResultBuilder::requests_reference)
    /// - [`history`](StatusHistoryResultBuilder::history)
    pub fn build(self) -> Result<StatusHistoryResult, BuildError> {
        Ok(StatusHistoryResult {
            requests_reference: self
                .requests_reference
                .ok_or_else(|| BuildError::missing_field("requests_reference"))?,
            history: self
                .history
                .ok_or_else(|| BuildError::missing_field("history"))?,
        })
    }
}
