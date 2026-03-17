pub use crate::prelude::*;

/// Response wrapper for declined v2 Money In transaction endpoints (HTTP 402). Returned when a transaction is declined by the card network or issuer. All decline responses use this format with unified response codes starting with 'D'. The `data` field contains transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct V2DeclinedTransactionResponseWrapper {
    pub code: V2ResponseCode,
    pub reason: V2ResponseReason,
    pub explanation: V2ResponseExplanation,
    pub action: V2ResponseAction,
    pub data: V2TransactionDetails,
    /// Pagination token (equivalent to `pageIdentifier` in v1 APIs). Returns `null` when pagination is not applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
