pub use crate::prelude::*;

/// Underwriting data is used to manage risk orchestration in the boarding application lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnderwritingDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<UnderWritingMethod>,
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<PolicyId>,
}
