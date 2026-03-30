pub use crate::prelude::*;

/// Underwriting data is used to manage risk orchestration in the boarding application lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnderwritingDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<UnderWritingMethod>,
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<PolicyId>,
}

impl UnderwritingDataResponse {
    pub fn builder() -> UnderwritingDataResponseBuilder {
        <UnderwritingDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnderwritingDataResponseBuilder {
    method: Option<UnderWritingMethod>,
    policy_id: Option<PolicyId>,
}

impl UnderwritingDataResponseBuilder {
    pub fn method(mut self, value: UnderWritingMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn policy_id(mut self, value: PolicyId) -> Self {
        self.policy_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnderwritingDataResponse`].
    pub fn build(self) -> Result<UnderwritingDataResponse, BuildError> {
        Ok(UnderwritingDataResponse {
            method: self.method,
            policy_id: self.policy_id,
        })
    }
}
