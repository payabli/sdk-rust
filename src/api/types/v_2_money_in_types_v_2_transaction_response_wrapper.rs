pub use crate::prelude::*;

/// Standard response wrapper for v2 Money In transaction endpoints. All v2 transaction endpoints return responses in this format with consistent `code`, `reason`, `explanation`, and `action` fields. The `data` field contains transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V2TransactionResponseWrapper {
    #[serde(default)]
    pub code: V2ResponseCode,
    #[serde(default)]
    pub reason: V2ResponseReason,
    #[serde(default)]
    pub explanation: V2ResponseExplanation,
    #[serde(default)]
    pub action: V2ResponseAction,
    pub data: V2TransactionDetails,
    /// Pagination token (equivalent to `pageIdentifier` in v1 APIs). Returns `null` when pagination is not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl V2TransactionResponseWrapper {
    pub fn builder() -> V2TransactionResponseWrapperBuilder {
        <V2TransactionResponseWrapperBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V2TransactionResponseWrapperBuilder {
    code: Option<V2ResponseCode>,
    reason: Option<V2ResponseReason>,
    explanation: Option<V2ResponseExplanation>,
    action: Option<V2ResponseAction>,
    data: Option<V2TransactionDetails>,
    token: Option<String>,
}

impl V2TransactionResponseWrapperBuilder {
    pub fn code(mut self, value: V2ResponseCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn reason(mut self, value: V2ResponseReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn explanation(mut self, value: V2ResponseExplanation) -> Self {
        self.explanation = Some(value);
        self
    }

    pub fn action(mut self, value: V2ResponseAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn data(mut self, value: V2TransactionDetails) -> Self {
        self.data = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V2TransactionResponseWrapper`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](V2TransactionResponseWrapperBuilder::code)
    /// - [`reason`](V2TransactionResponseWrapperBuilder::reason)
    /// - [`explanation`](V2TransactionResponseWrapperBuilder::explanation)
    /// - [`action`](V2TransactionResponseWrapperBuilder::action)
    /// - [`data`](V2TransactionResponseWrapperBuilder::data)
    pub fn build(self) -> Result<V2TransactionResponseWrapper, BuildError> {
        Ok(V2TransactionResponseWrapper {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            explanation: self
                .explanation
                .ok_or_else(|| BuildError::missing_field("explanation"))?,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            token: self.token,
        })
    }
}
