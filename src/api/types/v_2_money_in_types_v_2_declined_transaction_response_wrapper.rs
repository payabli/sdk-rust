pub use crate::prelude::*;

/// Response wrapper for declined v2 Money In transaction endpoints (HTTP 402). Returned when a transaction is declined by the card network or issuer. All decline responses use this format with unified response codes starting with 'D'. The `data` field contains transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct V2DeclinedTransactionResponseWrapper {
    #[serde(default)]
    pub code: V2ResponseCode,
    #[serde(default)]
    pub reason: V2ResponseReason,
    #[serde(default)]
    pub explanation: V2ResponseExplanation,
    #[serde(default)]
    pub action: V2ResponseAction,
    #[serde(default)]
    pub data: V2TransactionDetails,
    /// Pagination token (equivalent to `pageIdentifier` in v1 APIs). Returns `null` when pagination is not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl V2DeclinedTransactionResponseWrapper {
    pub fn builder() -> V2DeclinedTransactionResponseWrapperBuilder {
        <V2DeclinedTransactionResponseWrapperBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V2DeclinedTransactionResponseWrapperBuilder {
    code: Option<V2ResponseCode>,
    reason: Option<V2ResponseReason>,
    explanation: Option<V2ResponseExplanation>,
    action: Option<V2ResponseAction>,
    data: Option<V2TransactionDetails>,
    token: Option<String>,
}

impl V2DeclinedTransactionResponseWrapperBuilder {
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

    /// Consumes the builder and constructs a [`V2DeclinedTransactionResponseWrapper`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](V2DeclinedTransactionResponseWrapperBuilder::code)
    /// - [`reason`](V2DeclinedTransactionResponseWrapperBuilder::reason)
    /// - [`explanation`](V2DeclinedTransactionResponseWrapperBuilder::explanation)
    /// - [`action`](V2DeclinedTransactionResponseWrapperBuilder::action)
    /// - [`data`](V2DeclinedTransactionResponseWrapperBuilder::data)
    pub fn build(self) -> Result<V2DeclinedTransactionResponseWrapper, BuildError> {
        Ok(V2DeclinedTransactionResponseWrapper {
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
