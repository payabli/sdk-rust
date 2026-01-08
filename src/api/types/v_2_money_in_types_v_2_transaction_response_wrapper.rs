pub use crate::prelude::*;

/// Standard response wrapper for v2 Money In transaction endpoints. All v2 transaction endpoints return responses in this format with consistent `code`, `reason`, `explanation`, and `action` fields. The `data` field contains transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V2TransactionResponseWrapper {
    pub code: V2ResponseCode,
    pub reason: V2ResponseReason,
    pub explanation: V2ResponseExplanation,
    pub action: V2ResponseAction,
    pub data: V2TransactionDetails,
    /// Pagination token (equivalent to `pageIdentifier` in v1 APIs). Returns `null` when pagination is not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
